package core

import (
	"encoding/json"
	"errors"
	"os"

	"github.com/pelletier/go-toml"
)

type Config struct {
	Document struct {
		Title                   string      `toml:"title" json:"title"`
		Author                  interface{} `toml:"author" json:"author,omitempty"` // String or list of strings
		Date                    string      `toml:"date" json:"date,omitempty"`
		DocumentClass           string      `toml:"document-class" default:"article" json:"documentclass"`
		ClassOption             interface{} `toml:"class-option" json:"classoption"` // String or list of strings
		LatexHeader             string      `toml:"latex-header" json:"-"`
		HtmlHeader              string      `toml:"html-header" json:"-"`
		HeaderIncludes          string      `json:"header-includes"` // This only specifies output
		NumberSections          bool        `toml:"number-sections" json:"numbersections"`
        ReferenceSectionTitle   string      `toml:"references-title" json:"reference-section-title" default:"References"`
	} `toml:"document"`
	Build struct {
        Filename                string      `toml:"filename" default:"main"`
		Engine                  string      `toml:"engine" default:"tectonic"`
		LuaFilters              bool        `toml:"lua-filters" default:"true"`
		OutputFormat            string      `toml:"output-format" default:"pdf"`
	} `toml:"build"`
}

func (c *Config) WritePandocJson(path string) error {
	// Preprocessing
	if c.Document.Date == "today" {
		c.Document.Date = "\\today"
	}
	switch c.Build.OutputFormat {
	case "html":
		c.Document.HeaderIncludes = c.Document.HtmlHeader
	case "pdf":
		c.Document.HeaderIncludes = c.Document.LatexHeader
	}

	// Marshal config struct into JSON
	jsonBytes, err := json.Marshal(c.Document)
	if err != nil {
		return errors.New("Could not marshal metadata into JSON. " + err.Error())
	}

	// Write the JSON temporarily to path
	err = os.WriteFile(path, jsonBytes, 0644)
	if err != nil {
		return errors.New("Could not create JSON file. " + err.Error())
	}

	return nil
}

func ConfigFromFile(path string) (Config, error) {
	conf := Config{}
	tomlBytes, err := os.ReadFile(path)
	if err != nil {
		return conf, errors.New("Could not read config file. " + err.Error())
	}
	err = toml.Unmarshal(tomlBytes, &conf)
	if err != nil {
		return conf, errors.New("Could not unmarshal config file. " + err.Error())
	}
	return conf, nil
}
