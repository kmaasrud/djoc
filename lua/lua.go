package lua

import (
	_ "embed"
	"os"
	"path/filepath"
    "errors"

	"github.com/kmaasrud/doctor/utils"
	"github.com/kmaasrud/doctor/msg"
)

//go:embed crossref.lua
var crossref []byte

//go:embed abstract.lua
var abstract []byte

//go:embed wordcount.lua
var wordcount []byte

var buildFilters = map[string][]byte{
	"crossref.lua": crossref,
	"abstract.lua": abstract,
}

// Function returns a list of paths too all buildFilters. If the filters are not available in the
// Doctor data directory, then the function will ensure they are written there first.
func BuildFilters() []string {
    var paths []string

    dataDir, err := utils.FindDoctorDataDir()
    if err != nil {
        msg.Warning("Skipping Lua filters. Could not determine the Doctor data directory:\n    " + err.Error())
        return paths
    }

    err = utils.EnsureDir(dataDir)
    if err != nil {
        msg.Warning("Skipping Lua filters. Could not ensure Doctor data directory exists:\n    " + err.Error())
        return paths
    }

    for filename, filter := range buildFilters {
        path := filepath.Join(dataDir, filename)

        if _, err := os.Stat(path); os.IsNotExist(err) {
            msg.Info("Writing filter '" + filename + "' to '" + dataDir + "'.")
			err := os.WriteFile(path, filter, 0644)
			if err != nil {
				msg.Warning("Could not create '" + filename + "', skipping it. " + err.Error())
				continue
			}
        }

        paths = append(paths, path)
    }

    return paths
}

func WordCountFilter() (string, error) {
    dataDir, err := utils.FindDoctorDataDir()
    if err != nil {
        return "", errors.New("Could not determine the Doctor data directory.")
    }

    err = utils.EnsureDir(dataDir)
    if err != nil {
        return "", err
    }

    path := filepath.Join(dataDir, "wordcount.lua")

    if _, err := os.Stat(path); os.IsNotExist(err) {
        msg.Info("Writing filter 'wordcount.lua' to '" + dataDir + "'.")
        err := os.WriteFile(path, wordcount, 0644)
        if err != nil {
            return "", errors.New("Could not create 'wordcount.lua'. " + err.Error())
        }
    }
    
    return path, nil
}
