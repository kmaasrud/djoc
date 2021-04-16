package utils

import (
	"errors"
	"os"
	"path/filepath"
	"runtime"

	"github.com/kmaasrud/doctor/core"
	"github.com/kmaasrud/doctor/msg"
)

// Error type returned when no sections are found from the root path.
// Mainly used for not throwing an error in 'doctor add' when there are no sections,
// but also useful to specify messages.
type NoSectionsError struct {
	ErrorMsg string
}

func (e *NoSectionsError) Error() string {
	return e.ErrorMsg
}

// Searches up the directory tree to find a doctor.yaml file and returns the path
// of the directory containing it. If it reaches the root directory without finding
// anything, the function returns an error.
func FindDoctorRoot() (string, error) {
	path, err := os.Getwd()
	if err != nil {
		msg.Error(err.Error())
	}

	for {
		if filepath.Dir(path) == path {
			return "", errors.New("Could not find a Doctor document.")
		} else if _, err := os.Stat(filepath.Join(path, "doctor.toml")); os.IsNotExist(err) {
			path = filepath.Dir(path)
		} else {
			return path, nil
		}
	}
}

// Returns a slice containing core.Sections corresponding to this document
func FindSections(rootPath string) ([]core.Section, error) {
	var files []core.Section

	if _, err := os.Stat(filepath.Join(rootPath, "secs")); os.IsNotExist(err) {
		return nil, &NoSectionsError{"Empty Doctor document."}
	}
	// Walk should walk through dirs in lexical order, making sorting unecessary (luckily)
	err := filepath.Walk(filepath.Join(rootPath, "secs"), func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && filepath.Ext(path) == ".md" {
			sec, err := core.SectionFromPath(path)
			if err != nil {
				return err
			}
			files = append(files, sec)
		}
		return nil
	})
	if err != nil {
		return nil, err
	} else if len(files) < 1 {
		return nil, &NoSectionsError{"Empty Doctor document."}
	}

	return files, nil
}

// Returns the path where Doctor stores it's data. Supports both Windows and Unix.
func FindDoctorDataDir() (string, error) {
	var doctorPath string; var datadirEnv string; var defaultDir []string

	home, err := os.UserHomeDir()
	if err != nil {
		return doctorPath, err
	}

    if runtime.GOOS == "windows" {
        datadirEnv = "APPDATA"
        defaultDir = []string{home, "AppData", "Roaming", "doctor"}
    } else {
        datadirEnv = "XDG_DATA_DIR"
        defaultDir = []string{home, ".local", "share", "doctor"}
    }
    dataDir, exists := os.LookupEnv(datadirEnv)
    if exists {
        doctorPath = filepath.Join(dataDir, "doctor")
    } else {
        doctorPath = filepath.Join(defaultDir...)
    }

	return doctorPath, nil
}
