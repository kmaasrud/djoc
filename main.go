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

	rootCommand, _ := registry.Register("")
	rootCommand.AddFlag("check-dependencies", "", true, "")

	initCommand, _ := registry.Register("init")
	initCommand.AddFlag("default", "d", true, "")

	newCommand, _ := registry.Register("new")
	newCommand.AddArg("path", "")
	newCommand.AddFlag("default", "d", true, "")


    // Parse commands
	command, err := registry.Parse(os.Args[1:])
	// Handle command parsing errors
	if err != nil {
		if _, ok := err.(clapper.ErrorUnknownCommand); ok {
			msg.Error(fmt.Sprintf("Unknown command %s. Run %s to see a list of available commands.", msg.Style(os.Args[1], "Bold"), msg.Style("doctor --help", "Bold")))
		} else if _, ok := err.(clapper.ErrorUnknownFlag); ok {
			errorString := fmt.Sprintf("%s%s", strings.ToUpper(string(err.Error()[0])), string(err.Error()[1:]))
			msg.Error(fmt.Sprintf("%s. Run %s for further help.", errorString, msg.Style("kodb" + " --help", "Bold")))
		} else {
			msg.Error(err.Error())
		}
		os.Exit(1)
	}


	// Run the correct command logic
    commandLogic:
	switch command.Name {
	// Root command
	case "":
		for key, val := range command.Flags {
			switch key {
			case "check-dependencies":
				if val.Value == "true" { core.CheckDependencies() }
			}
		}

	case "init":
        for key := range command.Flags {
            switch key {
            case "default":
                core.CreateDocumentAt(".", true)
            }
			break commandLogic
        }
		core.CreateDocumentAt(".", false)

	case "new":
        var path string
		if val := command.Args["path"].Value; val != "" {
            path = val
		} else {
            path = command.Args["path"].DefaultValue
		}
        for key := range command.Flags {
            switch key {
            case "default":
                core.CreateDocumentAt(path, true)
            }
			break commandLogic
        }
        core.CreateDocumentAt(path, false)
	}
}
