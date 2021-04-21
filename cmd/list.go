package cmd

import (
    "errors"
    "fmt"

    "github.com/kmaasrud/doctor/utils"
)

func List() error {
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

    for _, sec := range secs {
        fmt.Println(sec.Title)
    }

    return nil
}
