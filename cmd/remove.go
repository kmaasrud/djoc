package cmd

import (
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/kmaasrud/doctor/core"
	"github.com/kmaasrud/doctor/msg"
	"github.com/kmaasrud/doctor/utils"
)

func Remove(inputs []string, confirm bool) error {
	var removeThis core.Section

	rootPath, err := utils.FindDoctorRoot()
	if err != nil {
		return errors.New("Could not remove section. " + err.Error())
	}

	// Find all existing sections
	secs, err := utils.FindSections(rootPath)
	if err != nil {
		if _, ok := err.(*utils.NoSectionsError); !ok {
			return errors.New("There are no sections in this document.")
		}
		return errors.New("Could not load section list. " + err.Error())
	}

	// Loop over supplied inputs and delete if they match
SectionLoop:
	for i, input := range inputs {
		var matches []core.Section
		index, err := strconv.Atoi(input)
		if err != nil {
			// The input is not parsable as int, handle it as a section name
			for _, sec := range secs {
				if strings.ToLower(sec.Title) == strings.ToLower(input) {
					matches = append(matches, sec)
				}
			}
		} else {
			// The input is parsable as int, handle it as a section index
			// Index matching is a bit difficult, since the indices change around a lot
			// when removing multiple sections. To solve this, subtract the number of sections
			// deleted from the index matched against.
			for _, sec := range secs {
				if sec.Index == index-i {
					matches = append(matches, sec)
				}
			}
		}

		if len(matches) == 1 {
			// Only one match, set is as the section to remove
			removeThis = matches[0]
		} else if len(matches) > 1 {
			// More than 1 match, enter interactive selection mode
			msg.Info(fmt.Sprintf("Found %d matches.", len(matches)))
			var chosenIndex string
			for true {
				for j, match := range matches {
					fmt.Printf(" %d. %s\n", j+1, match.Title)
				}
				fmt.Print("Which one do you want to delete? (q to quit) ")
				fmt.Scanln(&chosenIndex)
				index, err := strconv.Atoi(chosenIndex)
				if err == nil && index > 0 && index <= len(matches) {
					removeThis = matches[index-1]
					break
				} else if strings.ToLower(chosenIndex) == "q" {
					continue SectionLoop
				} else {
					msg.Info("That is not a valid option. Please enter the number of the section you want to remove.")
				}
			}
		} else {
			// No matches found
			msg.Error("Could not find any sections matching " + msg.Style(input, "Bold") + ".")
			continue SectionLoop
		}

		// Confirmation of deletion if not already supplied on the command line
		if !confirm {
			var confirmString string
			fmt.Printf("Are you sure you want to delete %s? (y/N) ", msg.Style(removeThis.Title, "Bold"))
			fmt.Scanln(&confirmString)
			if strings.ToLower(confirmString) != "y" {
				msg.Info("Skipping deletion of " + removeThis.Title + ".")
				continue SectionLoop
			}
		}

		// Remove the file
		err = os.Remove(removeThis.Path)
		if err != nil {
			msg.Error("Could not remove section " + msg.Style(removeThis.Title, "Bold") + ". " + err.Error())
			continue SectionLoop
		}
		msg.Success("Deleted section " + msg.Style(removeThis.Title, "Bold") + ".")

		// Decrement the sections above the removed one
		msg.Info("Reordering existing sections...")
		for j := removeThis.Index + 1; j < len(secs); j++ {
			// Make sure we're not trying to renumber removeThis itself (if multiple sections previously shared indices)
			if secs[j].IsEqual(removeThis) {
				continue
			}
			err = secs[j].ChangeIndex(j - 1)
			if err != nil {
				return errors.New("Could not bump index of existing section.\n        " + err.Error())
			}
		}

		if removeThis.Index > len(secs)-2 {
			// If the removed section has the highest index, just slice away the last element of secs
			secs = secs[:len(secs)-2]
		} else {
			// Else, remove the element pertaining to this index and keep the order by reslicing
			secs = append(secs[:removeThis.Index], secs[removeThis.Index+1:]...)
		}
	}

	return nil
}
