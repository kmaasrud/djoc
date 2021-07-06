package utils

import (
	"os"
    "os/exec"
	"path/filepath"
)

const ResourceSep string = ";"

// Returns the path where Doctor stores it's data on Windows.
func FindDoctorDataDir() (string, error) {
	var doctorPath string

	dataDir, exists := os.LookupEnv("APPDATA")
	if exists {
		doctorPath = filepath.Join(dataDir, "doctor")
	} else {
		home, err := os.UserHomeDir()
		if err != nil {
			return doctorPath, err
		}
		defaultDir := []string{home, "AppData", "Roaming", "doctor"}
		doctorPath = filepath.Join(defaultDir...)
	}

	return doctorPath, nil
}

func OpenFile(path string) error {
    cmd := exec.Command("start", path)
    err := cmd.Run()
    if err != nil {
        return err
    }

    return nil
}
