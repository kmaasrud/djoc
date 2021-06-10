package cmd

import (
	"bytes"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"

	"github.com/kmaasrud/doctor/core"
	"github.com/kmaasrud/doctor/core/bib"
	"github.com/kmaasrud/doctor/core/conf"
	"github.com/kmaasrud/doctor/lua"
	"github.com/kmaasrud/doctor/msg"
	"github.com/kmaasrud/doctor/utils"
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

func Build() error {
	// Check for dependencies
	_, err := utils.CheckPath("pandoc")
	if err != nil {
		return errors.New("Build failed. " + err.Error())
	}

	// Find root
	rootPath, err := utils.FindDoctorRoot()
	if err != nil {
		return errors.New("Build failed. " + err.Error())
	}

	// Initialize the command
	cmdArgs := []string{"-s"}

	// Find config
	msg.Info("Applying configuration from doctor.toml...")
	config, err := conf.ConfigFromFile(filepath.Join(rootPath, "doctor.toml"))
	if err != nil {
		return err
	}

	// Define output file
	cmdArgs = append(cmdArgs, "-o", filepath.Join(rootPath, config.Build.Filename+".pdf"))

	// Add resource paths
	resourcePaths := strings.Join([]string{rootPath, filepath.Join(rootPath, "assets"), filepath.Join(rootPath, "secs")}, utils.ResourceSep)
	cmdArgs = append(cmdArgs, "--resource-path="+resourcePaths)

	// Specify PDF engine and add options for specific engines
	pdfEngine, err := utils.CheckPath(config.Build.Engine)
	if err != nil {
		return errors.New("Build failed. " + err.Error())
	}
	cmdArgs = append(cmdArgs, fmt.Sprintf("--pdf-engine=%s", pdfEngine))
	if config.Build.Engine == "tectonic" {
		// Tectonic chatters a lot. Make it a bit more silent
		cmdArgs = append(cmdArgs, "--pdf-engine-opt=-c=minimal")
	}

	// Find source files
	msg.Info("Looking for source files...")
	secs, err := utils.FindSections(rootPath)
	if err != nil {
		return err
	}
	cmdArgs = append(cmdArgs, core.PathsFromSections(secs)...)
	plural := ""
	if len(secs) > 1 {
		plural = "s"
	}
	msg.Info(fmt.Sprintf("Found %d source file%s!", len(secs), plural))

	// Add Lua filters
	if config.Build.LuaFilters {
		msg.Info("Running with Lua filters...")
		for _, filter := range lua.Filters() {
			cmdArgs = append(cmdArgs, "-L", filter)
		}
	}

	// If references.bib exists, run with citeproc and add bibliography
	if f, err := os.Stat(filepath.Join(rootPath, "assets", config.Bib.BibliographyFile)); err == nil {
		msg.Info("Running with citeproc. Bibliography: '" + f.Name() + "'.")
		cmdArgs = append(cmdArgs, "-C", "--bibliography=references.bib")

		// If a CSL style is specified, make sure it exists in assets
		if cslName := config.Bib.Csl; cslName != "" {
			if val, ok := bib.Styles[cslName]; ok {
				err := os.WriteFile(filepath.Join(rootPath, "assets", cslName+".csl"), val, 0644)
				if err != nil {
					msg.Warning("Could not create CSL style, skipping it. " + err.Error())
					config.Bib.Csl = ""
				}
			}
		}
	} else if os.IsNotExist(err) {
		msg.Warning("Could not find bibliography file: '" + config.Bib.BibliographyFile + "'. Skipping citation processing.")
	}

	// Write Pandoc's config options into a JSON file
	jsonFilename := filepath.Join(rootPath, ".metadata.json")
	err = conf.WritePandocJson(jsonFilename, config)
	if err != nil {
		return err
	}
	cmdArgs = append(cmdArgs, "--metadata-file="+jsonFilename)
	defer cleanUpJson(rootPath)

	// Execute command
	done := make(chan struct{})
	go msg.Do("Building document with Pandoc", done)
	err = runPandocWith(cmdArgs)
	msg.CloseDo(done)

	// Handle errors
	if err != nil {
		var warnStr, errStr string
		switch thisErr := err.(type) {
		case *FatalError:
			_, errStr = msg.CleanStderrMsg(thisErr.Stderr)
			return errors.New("An error happened during build:\n\n" + errStr)
		case *WarningError:
			warnStr, _ = msg.CleanStderrMsg(thisErr.Stderr)
			msg.Success("Document built.")
			msg.Warning("A warning was thrown during build:\n\n" + warnStr)
			return errors.New("")
		default:
			return errors.New("Could not run command. " + err.Error())
		}
	}
	msg.Success("Document built.")
	return nil
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

func cleanUpJson(rootPath string) {
	err := os.Remove(filepath.Join(rootPath, ".metadata.json"))
	if err != nil {
		msg.Error("Failed to remove JSON metadata file. " + err.Error())
	}
}
