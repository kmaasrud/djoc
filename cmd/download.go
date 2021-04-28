package cmd

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/kmaasrud/doctor/utils"
)

// Checks if the dependency 'program' is present in PATH.
// TODO: Download Pandoc locally if not present. Perhaps with shell script
func CheckPath(program string) error {
	doctorPath, err := utils.FindDoctorDataDir()
	if err != nil {
		return err
	}

	// Check if dependency is available in the Doctor data dir
	// TODO: This is mainly here to allow backwards compatibility with those who have installed Pandoc 2.13
	// with Doctor previously
	if _, err := os.Stat(filepath.Join(doctorPath, program+"-2.13", "bin", program)); err == nil {
		return nil
	}
	_, err = exec.LookPath(program)
	if err != nil {
		return errors.New("Could not find " + program + " in your PATH.")
	}
	return nil
}
