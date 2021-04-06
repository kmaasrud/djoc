package core

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"

	"github.com/kmaasrud/doctor/msg"
	"github.com/kmaasrud/doctor/utils"
)

func Build() {
	// Check for dependencies
	err := utils.CheckDependencies()
	if err != nil {
		msg.Error("Build failed. " + err.Error())
		os.Exit(1)
	}

	// Find root
	rootPath, err := utils.FindDoctorRoot()
	if err != nil {
		msg.Error("Build failed. " + err.Error())
		os.Exit(1)
	}

	// Initialize command slice with options always present
	cmdArgs := []string{"-s", "--pdf-engine=tectonic", "--pdf-engine-opt=-c=minimal", "-o", "main.pdf"}

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
		msg.Error("Build failed! " + err.Error())
		os.Exit(1)
	} else if len(files) < 1 {
		msg.Warning("Could not find any source files. Aborting build.")
		os.Exit(0)
	}
	cmdArgs = append(cmdArgs, files...)
	msg.Info(fmt.Sprintf("Found %d source files!", len(files)))

	// Execute command
	done := make(chan struct{})
	go runPandocWith(cmdArgs, done)
	msg.Do("Building document with Pandoc", "Document built!", done)
}

func runPandocWith(cmdArgs []string, done chan struct{}) {
	var stderr bytes.Buffer
	cmd := exec.Command("pandoc", cmdArgs...)
	cmd.Stderr = &stderr

	err := cmd.Run()
	// Fatal error, exit with 1
	if err != nil {
		fmt.Print("\033[2K\r")
		if _, ok := err.(*exec.ExitError); ok {
			cleanStderrMsg(string(stderr.Bytes()))
		} else {
			msg.Error("Could not run command. " + err.Error())
		}
		os.Exit(1)
	}
	// Non-fatal, but stderr is not empty, so it includes warnings
	if stderr := string(stderr.Bytes()); stderr != "" {
		fmt.Print("\033[2K\r")
		cleanStderrMsg(stderr)
	}
	close(done)
}

// Tectonic, TeX and even Pandoc produces A LOT of noise. This function runs through each line
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
