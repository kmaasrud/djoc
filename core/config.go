package core

import (
	"encoding/json"
	"errors"
	"os"

	"github.com/pelletier/go-toml"
)

type BuildConfig struct {
	Engine			string			`toml:"engine" default:"tectonic"`
}

type DocumentConfig struct {
	Title			string			`toml:"title" json:"title"`
	Author			interface{}		`toml:"author" json:"author"` // String or list of strings
	Date			string			`toml:"date" json:"date"`
	DocumentClass	string			`toml:"document-class" json:"documentclass"`
	ClassOption		interface{}		`toml:"class-option" json:"classoption"` // String or list of strings
}

type Config struct {
	Document		DocumentConfig	`toml:"document"`
	Build			BuildConfig		`toml:"build"`
}

func (c *Config) WritePandocJson(path string) error {
	if c.Document.Date == "today" {
		c.Document.Date = "\\today"
	}
	jsonBytes, err := json.Marshal(c.Document)
	if err != nil {
		return errors.New("Could not marshal metadata into JSON. " + err.Error())
	}

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
