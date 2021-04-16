package cmd

import ( 
    "errors"
    "fmt"

    "github.com/kmaasrud/doctor/core"
    "github.com/kmaasrud/doctor/utils"
    "github.com/kmaasrud/doctor/msg"
)

func Move(input string, to int) error {
	rootPath, err := utils.FindDoctorRoot()
	if err != nil {
		return err
	}

	// Find all existing sections
	secs, err := utils.FindSections(rootPath)
	if err != nil {
		if _, ok := err.(*utils.NoSectionsError); !ok {
			return err
		}
		return errors.New("Could not load section list. " + err.Error())
	}

    // Find the section we want to move
    matches, err := core.FindSectionMatches(input, secs, 0)
    if err != nil {
        return err
    }

    // If multiple matches, enter interactive selection mode
    if len(matches) > 1 {
        var quit bool
        _, quit = msg.ChooseSection(matches, fmt.Sprintf("Found %d matches", len(matches)), "Which one do you want to move?")
        if quit {
            return nil
        }
    }
    return nil
}
