package utils

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
    "runtime"

	"github.com/kmaasrud/doctor/msg"
    "github.com/kmaasrud/doctor/core"
)

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
		msg.Error(fmt.Sprintf("%s", err))
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

// Returns a slice containing the paths of the source Markdown-files in the document.
func FindSections(rootPath string) ([]core.Section, error) {
    var files []core.Section

    if _, err := os.Stat(filepath.Join(rootPath, "secs")); os.IsNotExist(err) {
        return nil, &NoSectionsError{"Empty Doctor document.\n\tConsider adding a couple of source files with " + msg.Style("doctor add <section name>", "Bold")}
    }
	// Walk should walk through dirs in lexical order, making sorting unecessary (luckily)
	err := filepath.Walk(filepath.Join(rootPath, "secs"), func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && filepath.Ext(path) == ".md" {
			// TODO: Make sure the file ends in a couple of newlines (Lua filter?)
            sec, err := core.SectionFromPath(path)
            if err != nil {
                return err
            }
			files = append(files, sec)
		}
		return nil
	})
    if len(files) < 1 {
        return nil, &NoSectionsError{"Empty Doctor document.\n\tConsider adding a couple of source files with " + msg.Style("doctor add <section name>", "Bold")}
    } else if err != nil {
		return nil, err
	}

	return files, nil
}

// Returns the path where Doctor stores it's data. Supports both Windows and Unix.
// TODO: Accept variables like XDG_DATA_DIR and %DATADIR% (or whatever it's called on Windows).
func FindDoctorDataDir() (string, error) {
	home, err := os.UserHomeDir()
	if err != nil {
		return " ", err
	}

    var doctorPath string
    if runtime.GOOS == "windows" {
        doctorPath = filepath.Join(home, "AppData", "Roaming", "doctor")
    } else {
        doctorPath = filepath.Join(home, ".local", "share", "doctor")
    }

    return doctorPath, nil
}
