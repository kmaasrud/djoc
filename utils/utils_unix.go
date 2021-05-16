// +build aix darwin dragonfly freebsd js,wasm linux nacl netbsd openbsd solaris

package utils

import (
    "os"
    "path/filepath"
)

const ResourceSep string = ":"

// Returns the path where Doctor stores it's data. Supports both Windows and Unix.
func FindDoctorDataDir() (string, error) {
	var doctorPath string

    datadirEnv := "XDG_DATA_DIR"
	dataDir, exists := os.LookupEnv(datadirEnv)
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
