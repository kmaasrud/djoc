package cmd

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/kmaasrud/doctor/utils"
)

// Checks if the dependencies of Doctor are present.
func CheckDependencies() error {
	deps := map[string]string{"pandoc": "2.13"}

	doctorPath, err := utils.FindDoctorDataDir()
	if err != nil {
		return err
	}

	for dep, ver := range deps {
		// Check if dependency is available in the Doctor data dir
		if _, err := os.Stat(filepath.Join(doctorPath, dep+"-"+ver, "bin", dep)); err == nil {
			continue
		}
		_, err := exec.LookPath(dep)
		// TODO: Also check that the correct version is present
		if err != nil {
			// TODO: Download Pandoc locally if not present. Perhaps with shell script
			return errors.New("Could not find " + dep + " in your PATH.")
		}
	}
	return nil
}
