// +build aix dragonfly freebsd js,wasm linux nacl netbsd openbsd solaris

package utils

import (
	"os"
	"os/exec"
	"path/filepath"
)

const ResourceSep string = ":"

// Returns the path where Doctor stores it's data on Unix-like systems
func FindDoctorDataDir() (string, error) {
	var doctorPath string

	dataDir, exists := os.LookupEnv("XDG_DATA_DIR")
	if exists {
		doctorPath = filepath.Join(dataDir, "doctor")
	} else {
		home, err := os.UserHomeDir()
		if err != nil {
			return doctorPath, err
		}
		defaultDir := []string{home, ".local", "share", "doctor"}
		doctorPath = filepath.Join(defaultDir...)
	}

	return doctorPath, nil
}

func OpenFile(path string) error {
	var cmd *exec.Cmd

	editor, exists := os.LookupEnv("EDITOR")
	if exists {
		cmd = exec.Command(editor, path)
	} else {
		cmd = exec.Command("xdg-open", path)
	}

	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		return err
	}

	return nil
}
