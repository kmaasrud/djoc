package cmd

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
    _ "embed"

	"github.com/kmaasrud/doctor/msg"
	"github.com/kmaasrud/doctor/utils"
    "github.com/kmaasrud/doctor/lua"
    "github.com/kmaasrud/doctor/global"
    "github.com/kmaasrud/doctor/core"
)

type WarningError struct {
	Stderr string
}

func (e *WarningError) Error() string {
	return e.Stderr
}

type FatalError struct {
	Stderr string
}

func (e *FatalError) Error() string {
	return e.Stderr
}

func Build() {
	// Check for dependencies
	err := CheckDependencies()
	if err != nil {
		msg.Error("Build failed. " + err.Error())
        *global.ExitCode = 1; return
	}

	// Find root
	rootPath, err := utils.FindDoctorRoot()
	if err != nil {
		msg.Error("Build failed. " + err.Error())
        *global.ExitCode = 1; return
	}

    // Initialize the command
	cmdArgs := []string{"-s", "--pdf-engine=tectonic", "--pdf-engine-opt=-c=minimal", "-o", filepath.Join(rootPath, "main.pdf")}

	// Find source files
	msg.Info("Looking for source files...")
	secs, err := utils.FindSections(rootPath)
	if err != nil {
		msg.Error(err.Error())
        *global.ExitCode = 1; return
	} 

	cmdArgs = append(cmdArgs, core.PathsFromSections(secs)...)
	msg.Info(fmt.Sprintf("Found %d source files!", len(secs)))

    // Temporarily write any Lua filters to file and add them to command
    for filename, filter := range lua.Filters {
        err := os.WriteFile(filepath.Join(rootPath, filename), filter, 0644)
        if err != nil {
            msg.Error("Could not create Lua file. " + err.Error())
            *global.ExitCode = 1; return
        }
        cmdArgs = append(cmdArgs, "-L", filename)
    }
    defer cleanUpLuaFilters(rootPath)

	// If references.bib exists, run with citeproc and add bibliography
	if _, err := os.Stat(filepath.Join(rootPath, "assets", "references.bib")); err == nil {
        msg.Info("Running with citeproc. Bibliography: " + filepath.Join(rootPath, "assets", "references.bib"))
		cmdArgs = append(cmdArgs, "-C", "--bibliography=references.bib")
	}

	// Add resource paths
	var sep string
	if runtime.GOOS == "windows" {
		sep = ";"
	} else {
		sep = ":"
	}
	resourcePaths := strings.Join([]string{rootPath, filepath.Join(rootPath, "assets"), filepath.Join(rootPath, "src")}, sep)
	cmdArgs = append(cmdArgs, "--resource-path="+resourcePaths)


	// Execute command
	done := make(chan struct{})
	go msg.Do("Building document with Pandoc", done)
	err = runPandocWith(cmdArgs)
	msg.CloseDo(done)

    // Handle errors
	if err != nil {
        switch thisErr := err.(type) {
		case *FatalError:
			msg.CleanStderrMsg(thisErr.Stderr)
		case *WarningError:
			msg.CleanStderrMsg(thisErr.Stderr)
			msg.Success("Document built.")
		default:
			msg.Error("Could not run command. " + err.Error())
		}
        *global.ExitCode = 1; return
	}
	msg.Success("Document built.")
}

func runPandocWith(cmdArgs []string) error {
	var stderr bytes.Buffer
	cmd := exec.Command("pandoc", cmdArgs...)
	cmd.Stderr = &stderr

	err := cmd.Run()
	// Fatal error
	if err != nil {
		return &FatalError{string(stderr.Bytes())}
	}
	// Non-fatal, but stderr is not empty, so it includes warnings
	if stderr := string(stderr.Bytes()); len(stderr) != 0 {
		return &WarningError{string(stderr)}
	}
	return nil
}

func cleanUpLuaFilters(rootPath string) {
    if len(lua.Filters) > 0 {
        msg.Info("Cleaning up Lua filters...")
        for filename := range lua.Filters {
            err := os.Remove(filepath.Join(rootPath, filename))
            if err != nil {
                msg.Error("Failed to remove Lua filter " + filename + ". " + err.Error())
            }
        }
    }
}
