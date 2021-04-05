package main

import (
	"os"
	"fmt"
	"strings"
	"github.com/kmaasrud/doctor/msg"
	"github.com/kmaasrud/doctor/core"
	"github.com/thatisuday/clapper"
)

func main() {
	/* root, err := utils.FindDoctorRoot()
	if err != nil {
		msg.Error(err.Error())
		os.Exit(1)
	} */
	registry := clapper.NewRegistry()
	registry.Register("")

	registry.Register("init")
	newCommand, _ := registry.Register("new")
	newCommand.AddArg("path", "doctor_document")

	command, err := registry.Parse(os.Args[1:])
	if err != nil {
		if _, ok := err.(clapper.ErrorUnknownCommand); ok {
			msg.Error(fmt.Sprintf("Unknown command %s. Run %s to see a list of available commands.", msg.Style(os.Args[1], "Bold"), msg.Style("doctor --help", "Bold")))
		} else if _, ok := err.(clapper.ErrorUnknownFlag); ok {
			errorString := fmt.Sprintf("%s%s", strings.ToUpper(string(err.Error()[0])), string(err.Error()[1:]))
			msg.Error(fmt.Sprintf("%s. Run %s for a list of available flags", errorString, msg.Style(os.Args[1] + " --help", "Bold")))
		}
		os.Exit(1)
	}

	switch command.Name {
	case "init":
		core.CreateDocumentAt(".")
	case "new":
		if val := command.Args["path"].Value; val != "" {
			core.CreateDocumentAt(command.Args["path"].Value)
		} else {
			core.CreateDocumentAt(command.Args["path"].DefaultValue)
		}
	}
}
