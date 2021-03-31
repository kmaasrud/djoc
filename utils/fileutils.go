package utils

import (
    "os"
    "path/filepath"
    "errors"
)

// Searches up the directory tree to find a knutex.yaml file and returns the path
// of the directory containing it. If it reaches the root directory without finding
// anything, FindKnutexRoot returns an error.
func FindKodbRoot() (string, error) {
    path, err := os.Getwd(); CheckErr(err)

    for {
        if filepath.Dir(path) == path {
            return "", errors.New("Could not find a KnuTeX root")
        } else if _, err := os.Stat(filepath.Join(path, "knutex.yaml")); os.IsNotExist(err) {
            path = filepath.Dir(path)
        } else {
            return path, nil
        }
    }
}
