package lua

import (
	_ "embed"
	"os"
	"path/filepath"

	"github.com/kmaasrud/doctor/utils"
	"github.com/kmaasrud/doctor/msg"
)

//go:embed crossref.lua
var crossref []byte

//go:embed abstract.lua
var abstract []byte

//go:embed wordcount.lua
var wordcount []byte

var filters = map[string][]byte{
	"crossref.lua": crossref,
	"abstract.lua": abstract,
}

func Filters() []string {
    var paths []string

    dataDir, _ := utils.FindDoctorDataDir()
    for filename, filter := range filters {
        path := filepath.Join(dataDir, filename)

        if _, err := os.Stat(path); os.IsNotExist(err) {
            msg.Info("Writing filter '" + filename + "' to '" + dataDir + "'.")
			err := os.WriteFile(path, filter, 0644)
			if err != nil {
				msg.Warning("Could not create Lua file, skipping it. " + err.Error())
				continue
			}
        }

        paths = append(paths, path)
    }

    return paths
}
