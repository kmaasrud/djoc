package cmd

import (
    "errors"
    "fmt"

    "github.com/kmaasrud/doctor/utils"
)

func Add(sectionName string, index int) error {
    rootPath, err := utils.FindDoctorRoot()
    if err != nil { 
        return errors.New("Could not add a new section. " + err.Error())
    }

	files, err := utils.FindSections(rootPath)
    if _, ok := err.(*utils.NoSectionsError); !ok && err != nil {
        return errors.New("Could not add a new section. " + err.Error())
	}

    for _, file := range files {
        fmt.Println(file)
    }
    return nil
}
