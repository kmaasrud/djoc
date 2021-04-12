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

func Remove(input string, confirm bool) error {
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

    index, err := strconv.Atoi(input)
    if err != nil {
        // The input is not parsable as int, handle it as a section name
        for _, sec := range secs {
            if strings.ToLower(sec.Title) == input {
                removeThis = sec
                break
            } 
        }
    } else {
        // The input is parsable as int, handle it as a section index
        removeThis = secs[index]
    }

    // Remove the file
    err = os.Remove(removeThis.Path)
    if err != nil {
        return errors.New("Could not remove section " + msg.Style(removeThis.Title, "Bold") + ". " + err.Error())
    }

    // Decrement the sections above the removed one
    msg.Info("Reordering existing sections...")
    for i := removeThis.Index + 1; i < len(secs); i++ {
        err := secs[i].ChangeIndex(i-1)
        if err != nil {
            return errors.New("Could not bump index of existing section. " + err.Error())
        }
    }

    msg.Success("Deleted section " + msg.Style(removeThis.Title, "Bold") + ".")
    return nil
}
