package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	_ "embed"

	"github.com/kmaasrud/doctor/core"
	"github.com/kmaasrud/doctor/msg"
    "github.com/kmaasrud/doctor/global"
	"github.com/thatisuday/clapper"
)

func main() {
	registry := clapper.NewRegistry()

	rootCommand, _ := registry.Register("")
	rootCommand.AddFlag("dependencies", "", true, "")

	newCommand, _ := registry.Register("new")
	newCommand.AddArg("path", ".")
	newCommand.AddFlag("default", "d", true, "")

	registry.Register("build")

	addCommand, _ := registry.Register("add")
	addCommand.AddArg("name", "")

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
		// Can discard this err, command.Flags["dependencies"].Value will always be a parsable bool
		if ok, _ := strconv.ParseBool(command.Flags["dependencies"].Value); ok {
			err := core.CheckDependencies()
			if err != nil {
				msg.Error(err.Error())
				*global.ExitCode = 1; break
			}
			msg.Success("All the dependencies are installed. You're ready to go!")
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
	case "add":
		if command.Args["name"].Value == "" {
			msg.Error("Please supply a name for your section.")
			*global.ExitCode = 1; break
		}
		core.Add(command.Args["name"].Value)
	}
    os.Exit(*global.ExitCode)
}
