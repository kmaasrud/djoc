package cmd

import (
    "fmt"
    "os"
    "strconv"
    
    "github.com/thatisuday/clapper"
)

func Help(helpFlag *clapper.Flag, cmdName string) {
    if ok, _ := strconv.ParseBool(helpFlag.Value); ok {
        fmt.Println(helpText[cmdName]) 
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
    
    "add": `test`,
}
