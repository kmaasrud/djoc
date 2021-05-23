package utils

import (
	"os"
	"path/filepath"
)

const ResourceSep string = ";"

// Returns the path where Doctor stores it's data. Supports both Windows and Unix.
func FindDoctorDataDir() (string, error) {
	var doctorPath string

	datadirEnv := "APPDATA"
	dataDir, exists := os.LookupEnv(datadirEnv)
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
