package utils

import (
    "os"
    "fmt"
    "path/filepath"
    "errors"
    "github.com/kmaasrud/doctor/msg"
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
            return "", errors.New("Could not find a " + msg.Style("Doctor", "bold") + " document")
        } else if _, err := os.Stat(filepath.Join(path, "doctor.yaml")); os.IsNotExist(err) {
            path = filepath.Dir(path)
        } else {
            return path, nil
        }
    }
}
