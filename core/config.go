package core

import (
	"encoding/json"
	"errors"
	"os"

	"github.com/pelletier/go-toml"
)

var pandocConfig = map[string]string{
	"document.title":  "title",
	"document.author": "author",
	"document.date":   "date",
}

type Config struct {
	Tree *toml.Tree
}

func (c *Config) WritePandocJson(path string) error {
	metaMap := make(map[string]interface{})
	for key, metadataName := range pandocConfig {
		if c.Tree.Has(key) {
			metaMap[metadataName] = c.Tree.Get(key)
		}
	}

	jsonStr, err := json.Marshal(metaMap)
	if err != nil {
		return errors.New("Could not marshal metadata into JSON. " + err.Error())
	}

	err = os.WriteFile(path, jsonStr, 0644)
	if err != nil {
		return errors.New("Could not create JSON file. " + err.Error())
	}
	return nil
}

func ConfigFromFile(path string) (Config, error) {
	tree, err := toml.LoadFile(path)
	if err != nil {
		return Config{}, errors.New("Could not load config file. " + err.Error())
	}

	return Config{tree}, nil
}
