package cmd

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"

	"github.com/kmaasrud/doctor/msg"
	"github.com/kmaasrud/doctor/utils"
)

const tomlFile string = `[meta]
title = "%s"
author = "AUTHOR"
date = "today"
`

func CreateDocumentAt(path string, defaultStructure bool) error {
	rootPath, err := filepath.Abs(path)
	if err != nil {
		return err
	}

	// Check if specified directory exists. If not, create it. If it does, just write into it
	if _, existErr := os.Stat(rootPath); os.IsNotExist(existErr) {
		err := os.Mkdir(rootPath, 0755)
		if err != nil {
			return errors.New("Could not create root directory: " + err.Error())
		}
		msg.Info("Created new directory " + msg.Style(rootPath, "Bold") + ".")
	}

	// Create the assets directory if it doesn't exist. If it does, just write into it
	assetPath := filepath.Join(rootPath, "assets")
	if _, existErr := os.Stat(assetPath); os.IsNotExist(existErr) {
		err := os.Mkdir(assetPath, 0755)
		if err != nil {
			return errors.New("Could not create assets directory: " + err.Error())
		}
		msg.Info("Made " + msg.Style("assets", "Bold") + " directory.")
	} else {
		msg.Info("The assets directory already exists, keeping it.")
	}

	// Find name of document's directory, and declare the title as the capitalized version of it
	docTitle := utils.CapitalizeFirst(filepath.Base(rootPath))
	// Create the TOML config file with rw permissions for all
	tomlPath := filepath.Join(rootPath, "doctor.toml")
	err = ioutil.WriteFile(tomlPath, []byte(fmt.Sprintf(tomlFile, docTitle)), 0666)
	if err != nil {
		return errors.New("Unable to write file: " + err.Error())
	} else {
		msg.Info("Created config file: " + filepath.Base(tomlPath) + ".")
	}

	// Create bibliography file with rw permissions for all.
	// TODO: Consider not adding this on 'doctor new', but only when required.
	refPath := filepath.Join(assetPath, "references.bib")
	err = ioutil.WriteFile(refPath, []byte(""), 0666)
	if err != nil {
		return errors.New("Unable to write file: " + err.Error())
	} else {
		msg.Info("Created bibliography file: " + filepath.Base(refPath) + ".")
	}

	// TODO: Make this functional
	if defaultStructure {
		msg.Info("Adding sections of default document structure...")
	}

	return nil
}
