package core

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
)

type WarningError struct {
	Stderr string
}

func (e *WarningError) Error() string {
	return e.Stderr
}

type ExitError struct {
	Stderr string
}

func (e *ExitError) Error() string {
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

	// Initialize command slice with options always present
	cmdArgs := []string{"-s", "--pdf-engine=tectonic", "--pdf-engine-opt=-c=minimal", "-o", filepath.Join(rootPath, "main.pdf")}

    // Temporarily write any Lua filters to file and add them to command
    for filename, filter := range lua.Filters {
        f, err := os.Create(filepath.Join(rootPath, filename))
        if err != nil {
            msg.Error("Build failed. " + err.Error())
            *global.ExitCode = 1; return
        }
        _, err = f.Write(filter)
        if err != nil {
            msg.Error("Could not write Lua file. " + err.Error())
            *global.ExitCode = 1; return
        }
        cmdArgs = append(cmdArgs, "-L", filename)
    }
    defer cleanUpLuaFilters(rootPath)

	// If references.bib exists, run with citeproc and add bibliography
	if _, err := os.Stat(filepath.Join(rootPath, "assets", "references.bib")); err == nil {
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

	// Find source files
	msg.Info("Looking for source files...")
	files, err := utils.FindSrcFiles(rootPath)
	if err != nil {
		msg.Error(err.Error())
        *global.ExitCode = 1; return
	} 

	cmdArgs = append(cmdArgs, files...)
	msg.Info(fmt.Sprintf("Found %d source files!", len(files)))

	// Execute command
	done := make(chan struct{})
	go msg.Do("Building document with Pandoc", done)
	err = runPandocWith(cmdArgs)
	msg.CloseDo(done)
	if err != nil {
		switch err.(type) {
		case *ExitError:
			cleanStderrMsg(err.(*ExitError).Stderr)
		case *WarningError:
			cleanStderrMsg(err.(*WarningError).Stderr)
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
	// Fatal error, send the error over the channel
	if err != nil {
		return &ExitError{string(stderr.Bytes())}
	}
	// Non-fatal, but stderr is not empty, so it includes warnings
	if stderr := string(stderr.Bytes()); len(stderr) != 0 {
		return &WarningError{string(stderr)}
	}
	return nil
}

// Tectonic, TeX and even Pandoc produce A LOT of noise. This function runs through each line
// of stderr and returns only those containing relevant information. This cleans up a lot and
// allows me to style the errors/warnings according to Doctor messages. I admit it might be a bit
// stupid, since I can never be sure to catch everything, but I think it is worth the debug time.
func cleanStderrMsg(stderr string) {
	includeNext := false
	for _, line := range strings.Split(strings.TrimSuffix(stderr, "\n"), "\n") {
		if includeNext {
			fmt.Println("         " + line)
			includeNext = false
		} else if strings.HasPrefix(line, "! ") {
			msg.Error(msg.Style("TeX: ", "Bold") + strings.TrimPrefix(line, "! "))
			includeNext = true
		} else if strings.HasPrefix(line, "error: ") {
			msg.Error(msg.Style("Tectonic: ", "Bold") + strings.TrimPrefix(line, "error: "))
		} else if strings.HasPrefix(line, "[WARNING] ") {
			msg.Warning(msg.Style("Pandoc: ", "Bold") + strings.TrimPrefix(line, "[WARNING] "))
		} else if strings.HasPrefix(line, "[ERROR] ") {
			msg.Error(msg.Style("Pandoc: ", "Bold") + strings.TrimPrefix(line, "[ERROR] "))
		}
	}
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
