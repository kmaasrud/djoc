package main

import (
	_ "embed"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/kmaasrud/doctor/cmd"
	"github.com/kmaasrud/doctor/msg"
	"github.com/thatisuday/clapper"
)

var VERSION = "DEV"

func main() {
	registry := clapper.NewRegistry()

	rootCommand, _ := registry.Register("")
	rootCommand.AddFlag("dependencies", "", true, "")
	rootCommand.AddFlag("version", "v", true, "")
	rootCommand.AddFlag("update", "u", true, "")

	newCommand, _ := registry.Register("new")
	newCommand.AddArg("path", ".")
	newCommand.AddFlag("default", "d", true, "")

	registry.Register("build")

	addCommand, _ := registry.Register("add")
	addCommand.AddArg("name", "")
	addCommand.AddFlag("at", "i", false, "")

	removeCommand, _ := registry.Register("remove")
	removeCommand.AddArg("sections...", "")
	removeCommand.AddFlag("confirm", "c", true, "")

	moveCommand, _ := registry.Register("move")
	moveCommand.AddArg("section", "")
	moveCommand.AddArg("to", "")

	// Parse commands
	command, err := registry.Parse(os.Args[1:])
	// Handle command parsing errors
	if err != nil {
		if _, ok := err.(clapper.ErrorUnknownCommand); ok {
			msg.Error("Unknown command " + msg.Style(os.Args[1], "Bold") + ". Run " + msg.Style("doctor --help", "Bold") + " to see a list of available commands.")
		} else if _, ok := err.(clapper.ErrorUnknownFlag); ok {
			errorString := strings.ToUpper(string(err.Error()[0])) + string(err.Error()[1:])
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
		for flag, val := range command.Flags {
			// Can discard this err, the root command's flags will always be parsable bools
			if ok, _ := strconv.ParseBool(val.Value); ok {
				switch flag {
				case "dependencies":
					err := cmd.CheckDependencies()
					if err != nil {
						msg.Error(err.Error())
						os.Exit(1)
					}
					msg.Success("All the dependencies are installed. You're ready to go!")

				case "version":
					fmt.Println("You are running Doctor " + VERSION)

				case "update":
					err := cmd.Update()
					if err != nil {
						msg.Error(err.Error())
						os.Exit(1)
					}
				}
			}
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

		err := cmd.CreateDocumentAt(path, makeDefault)
		if err != nil {
			msg.Error(err.Error())
			os.Exit(1)
		}

	// Build the document command
	case "build":
		err := cmd.Build()
		if err != nil {
			// Some error messages are handled within the build function and just return an empty error
			if err.Error() != "" {
				msg.Error(err.Error())
			}
			os.Exit(1)
		}

	// Add a new section to the document
	case "add":
		var err error
		if command.Args["name"].Value == "" {
			msg.Error("Please supply a name for your section.")
			os.Exit(1)
		} else if indexString := command.Flags["at"].Value; indexString != "" {
			index, err := strconv.Atoi(indexString)
			if err != nil {
				msg.Error("Could not parse index: " + indexString + ". " + err.Error())
				os.Exit(1)
			}
			err = cmd.Add(command.Args["name"].Value, index)
		} else {
			err = cmd.Add(command.Args["name"].Value, -1)
			if err != nil {
				msg.Error(err.Error())
				os.Exit(1)
			}
		}

	// Remove a section from the document
	case "remove":
		if command.Args["sections"].Value == "" {
			msg.Error("Please supply the name or index of the section(s) you want to remove.")
			os.Exit(1)
		}

		// Can discard this err, command.Flags["confirm"].Value will always be a parsable bool
		confirm, _ := strconv.ParseBool(command.Flags["confirm"].Value)

		err := cmd.Remove(strings.Split(command.Args["sections"].Value, ","), confirm)
		if err != nil {
			msg.Error(err.Error())
			os.Exit(1)
		}

	// Move a section from one position to another
	case "move":
		section := command.Args["section"].Value
		toStr := command.Args["to"].Value
		if section == "" || toStr == "" {
			msg.Error("Please supply the section you want to move and the index you want to move it to.")
			os.Exit(1)
		}

		to, err := strconv.Atoi(toStr)
		if err != nil {
			msg.Error("Could not parse " + toStr + " as an index. Please supply a valid number.")
		}

		err = cmd.Move(section, to)
		if err != nil {
			msg.Error(err.Error())
			os.Exit(1)
		}
	}
}
