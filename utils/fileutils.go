package utils

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/kmaasrud/doctor/core"
	"github.com/kmaasrud/doctor/msg"
)

// Error type returned when no sections are found from the root path.
// Mainly used for not throwing an error in 'doctor add' when there are no sections,
// but also useful to specify messages.
type NoSectionsError struct {
	errorMsg string
}

func (e *NoSectionsError) Error() string {
	return e.errorMsg
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

// Wrapper function around exec.LookPath. Also consults the Doctor data dir
func CheckPath(program string) (string, error) {
	// Check for program in Doctor's data directory
	doctorPath, err := FindDoctorDataDir()
	if err == nil {
		path := filepath.Join(doctorPath, "bin", program)
		if _, err := os.Stat(path); err == nil {
			return path, nil
		}
	}

	// Check if program in PATH
	path, err := exec.LookPath(program)
	if err != nil {
		return "", errors.New("Could not find " + program + " in your PATH.")
	}
	return path, nil
}

// Ensures a directory exists. If not, creates it (and any needed parents.)
func EnsureDir(path string) error {
	if _, err := os.Stat(path); os.IsNotExist(err) {
		err := os.MkdirAll(path, os.ModePerm)
		if err != nil {
			return err
		}
	}
	return nil
}
