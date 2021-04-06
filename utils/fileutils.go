package utils

import (
	"errors"
	"fmt"
	"github.com/kmaasrud/doctor/msg"
	"os"
	"os/exec"
	"path/filepath"
)

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
func FindSrcFiles(rootPath string) ([]string, error) {
	var files []string

	// Walk should walk through dirs in lexical order, making sorting unecessary (luckily)
	err := filepath.Walk(filepath.Join(rootPath, "src"), func(path string, info os.FileInfo, err error) error {
		if err != nil {
			msg.Error(err.Error())
		}
		if !info.IsDir() && filepath.Ext(path) == ".md" {
			// TODO: Make sure the file ends in a couple of newlines
			files = append(files, path)
		}
		return nil
	})
	if err != nil {
		return files, err
	}

	return files, nil
}

// Checks if the dependencies of Doctor are present.
func CheckDependencies() error {
	deps := [2]string{"pandoc", "tectonic"}
	for _, dep := range deps {
		_, err := exec.LookPath(dep)
		if err != nil {
			return errors.New("Could not find " + msg.Style(dep, "Bold") + " in your PATH.")
		}
	}
	return nil
}
