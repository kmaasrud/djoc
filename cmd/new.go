package cmd

import (
	"fmt"
	"github.com/kmaasrud/doctor/msg"
	"io/ioutil"
	"os"
	"path/filepath"
)

const tomlFile string = `title = "TITLE"
author = "AUTHOR"
date = "today"
`

func CreateDocumentAt(path string, defaultStructure bool) {
	rootPath, err := filepath.Abs(path)
	if err != nil {
		msg.Error(err.Error())
	}

	if _, existErr := os.Stat(rootPath); os.IsNotExist(existErr) {
		err := os.Mkdir(rootPath, 0777)
		if err != nil {
			msg.Error("Could not create root directory: " + err.Error())
		}
		msg.Info(fmt.Sprintf("Made new Doctor document in %s", msg.Style(rootPath, "Bold")))
	} else {
		msg.Info(fmt.Sprintf("The directory %s already exists, creating document there...", msg.Style(rootPath, "Bold")))
	}

	assetPath := filepath.Join(rootPath, "assets")
	if _, existErr := os.Stat(assetPath); os.IsNotExist(existErr) {
		err := os.Mkdir(assetPath, 0777)
		if err != nil {
			msg.Error("Could not create assets directory: " + err.Error())
		}
		msg.Info("Made " + msg.Style("assets", "Bold") + " directory.")
	} else {
		msg.Info(fmt.Sprintf("The directory %s already exists, keeping it.", msg.Style(assetPath, "Bold")))
	}

	tomlPath := filepath.Join(rootPath, "doctor.toml")
	err = ioutil.WriteFile(tomlPath, []byte(tomlFile), 0666)
	if err != nil {
		msg.Error("Unable to write file: " + err.Error())
	} else {
		msg.Info("Created " + msg.Style(filepath.Base(tomlPath), "Bold"))
	}

	refPath := filepath.Join(assetPath, "references.bib")
	err = ioutil.WriteFile(refPath, []byte(""), 0666)
	if err != nil {
		msg.Error("Unable to write file: " + err.Error())
	} else {
		msg.Info("Created " + msg.Style(filepath.Base(refPath), "Bold"))
	}

	if defaultStructure {
		msg.Info("Creating default document structure")
	}
}
