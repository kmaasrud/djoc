package cmd

import (
    "strconv"
    "errors"
    "strings"
    "os"

    "github.com/kmaasrud/doctor/utils"
    "github.com/kmaasrud/doctor/core"
    "github.com/kmaasrud/doctor/msg"
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
                if sec.Index == index - i {
                    matches = append(matches, sec)
                } 
            }
        }

        if len(matches) == 1 {
            // Only one match, set is as the section to remove
            removeThis = matches[0]
        } else if len(matches) > 1 {
            // TODO: More than 1 match, enter interactive selection mode
        } else {
            // No matches found
            msg.Error("Could not find any sections matching " + msg.Style(input, "Bold") + ".")
            continue
        }

        // Remove the file
        err = os.Remove(removeThis.Path)
        if err != nil {
            msg.Error("Could not remove section " + msg.Style(removeThis.Title, "Bold") + ". " + err.Error())
            continue
        }

        msg.Success("Deleted section " + msg.Style(removeThis.Title, "Bold") + ".")

        // Decrement the sections above the removed one
        msg.Info("Reordering existing sections...")
        for j := removeThis.Index + 1; j < len(secs); j++ {
            err := secs[j].ChangeIndex(j-1)
            if err != nil {
                return errors.New("Could not bump index of existing section.\n        " + err.Error())
            }
        }

        secs = append(secs[:removeThis.Index], secs[removeThis.Index+1:]...)
    }

    return nil
}
