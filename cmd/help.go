package cmd

import (
	"fmt"
	"os"
	"strconv"

	"github.com/thatisuday/clapper"
)

func Help(helpFlag *clapper.Flag, cmdName string) {
	if ok, _ := strconv.ParseBool(helpFlag.Value); ok {
		fmt.Println("\n" + helpText[cmdName] + "\n")
		os.Exit(0)
	}
}

var helpText = map[string]string{
	"": `Doctor is a command line tool and environment for building scientific documents.

Usage:
    doctor <command> [arguments]

Commands:
    add             Add a section
    build           Build the document
    list            List the document structure
    move            Move a section
    new             Creates a new document
    remove          Remove a section

Flags:
    --dependencies  Checks if all dependencies are installed
    --help          Shows this text
    --update        Checks if an update is available and offers to download it
    --version       Prints the current version`,

	// ---
	"add": `Add a new section to your Doctor document.

Usage:
    doctor add <section name> [options]

Options:
    --at            Specify which index the new section should have`,

	// ---
	"build": `Builds the current document according to doctor.toml`,

	// ---
	"new": `Create a new Doctor document in the specified location.

Usage:
	doctor new [<directory>] [options]

[<directory>] is an optional argument specifying either an existing directory,
or the name of a new one. If the argument is omitted, Doctor will create a
document in the current directory.

Options:
    --default       Initialize the document with a standard report type
                    document structure. Very limited at the moment.`,
}
