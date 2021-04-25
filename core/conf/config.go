package conf

import (
	"errors"
	"os"

	"github.com/pelletier/go-toml"
)

type Config struct {
	Build struct {
        Filename                string      `toml:"filename" default:"document"`
		Engine                  string      `toml:"engine" default:"pdflatex"`
		LuaFilters              bool        `toml:"lua-filters" default:"true"`
		OutputFormat            string      `toml:"output-format" default:"pdf"`
	} `toml:"build"`
    Meta struct {
        Title                   string      `toml:"title"`
        Author                  interface{} `toml:"author"`
        Date                    string      `toml:"date"` // String or list of strings
    } `toml:"meta"`
    Style struct {
		DocumentClass           string      `toml:"document-class" default:"article"`
		ClassOptions            interface{} `toml:"class-options"` // String or list of strings
        NumberSections          bool        `toml:"number-sections" default:"false"`
    } `toml:"style"`
    Bib struct {
        ReferencesTitle         string      `toml:"references-title"`
        Csl                     string      `toml:"csl"` // Default is Chicago MoS 17th Ed.
        LinkCitations           bool        `toml:"link-citations" default:"true"`
    } `toml:"bib"`
    Latex struct {
		Header                  string      `toml:"header"`
        Packages                []string    `toml:"packages"`
    } `toml:"latex"`
    Html struct {
		Header                  string      `toml:"header"`
    } `toml:"html"`
}

func ConfigFromFile(path string) (*Config, error) {
	conf := Config{}
    // Load the TOML bytes
	tomlBytes, err := os.ReadFile(path)
	if err != nil {
		return &conf, errors.New("Could not read config file. " + err.Error())
	}

    // Unmarshal into config struct
	err = toml.Unmarshal(tomlBytes, &conf)
	if err != nil {
		return &conf, errors.New("Could not unmarshal config file. " + err.Error())
	}

	return &conf, nil
}
