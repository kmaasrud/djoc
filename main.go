package main

import (
	"fmt"
	"github.com/kmaasrud/doctor/core"
	"github.com/kmaasrud/doctor/msg"
	"github.com/thatisuday/clapper"
	"os"
	"strconv"
	"strings"
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

	newCommand, _ := registry.Register("new")
	newCommand.AddArg("path", ".")
	newCommand.AddFlag("default", "d", true, "")

	registry.Register("build")

	// Parse commands
	command, err := registry.Parse(os.Args[1:])
	// Handle command parsing errors
	if err != nil {
		if _, ok := err.(clapper.ErrorUnknownCommand); ok {
			msg.Error(fmt.Sprintf("Unknown command %s. Run %s to see a list of available commands.", msg.Style(os.Args[1], "Bold"), msg.Style("doctor --help", "Bold")))
		} else if _, ok := err.(clapper.ErrorUnknownFlag); ok {
			errorString := fmt.Sprintf("%s%s", strings.ToUpper(string(err.Error()[0])), string(err.Error()[1:]))
			msg.Error(fmt.Sprintf("%s. Run %s for further help.", errorString, msg.Style("kodb"+" --help", "Bold")))
		} else {
			msg.Error(err.Error())
		}
		os.Exit(1)
	}

	// Run the correct command logic
	switch command.Name {
	// Root command
	case "":
		// Can discard this err, command.Flags["check-dependencies"].Value will always be a parsable bool
		if ok, _ := strconv.ParseBool(command.Flags["check-dependencies"].Value); ok {
			core.CheckDependencies()
		}

	// Create new document command
	case "new":
		var path string
		if val := command.Args["path"].Value; val != "" {
			path = val
		} else {
			path = command.Args["path"].DefaultValue
		}
		// Can discard this err, command.Flags["default"].Value will always be a parsable bool
		makeDefault, _ := strconv.ParseBool(command.Flags["default"].Value)
		core.CreateDocumentAt(path, makeDefault)

	case "build":
		core.Build()
	}
}
