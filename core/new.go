package core

import (
	"github.com/kmaasrud/doctor/msg"
	"os"
    "fmt"
	"path/filepath"
	"io/ioutil"
)

const yamlFile string = `title: "TITLE"
author: "AUTHOR"
date: \today

# Bibliography
reference-section-title: "References"
bibliography: references.bib
`

func CreateDocumentAt(path string) {
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

	yamlPath := filepath.Join(rootPath, "doctor.yaml")
	err = ioutil.WriteFile(yamlPath, []byte(yamlFile), 0666)
    if err != nil {
        msg.Error("Unable to write file: " + err.Error())
	} else {
		msg.Info("Created " + msg.Style(filepath.Base(yamlPath), "Bold"))
	}

	refPath := filepath.Join(assetPath, "references.bib")
	err = ioutil.WriteFile(refPath, []byte(""), 0666)
    if err != nil {
        msg.Error("Unable to write file: " + err.Error())
	} else {
		msg.Info("Created " + msg.Style(filepath.Base(refPath), "Bold"))
	}
}
