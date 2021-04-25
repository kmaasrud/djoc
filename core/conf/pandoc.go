package conf

import (
    "errors"
    "os"
    "encoding/json"
)

type PandocConfig struct {
    Title           string      `json:"title,omitempty"`
    Author          interface{} `json:"author,omitempty"`
    Date            string      `json:"date"`
    DocumentClass   string      `json:"documentclass"`
    ClassOptions    interface{} `json:"classoption,omitempty"`
    NumberSections  bool        `json:"numbersections"`
    ReferencesTitle string      `json:"reference-section-title,omitempty"`
    Csl             string      `json:"csl,omitempty"`
    LinkCitations   bool        `json:"link-citations"`
	HeaderIncludes  string      `json:"header-includes,omitempty"`
}

func WritePandocJson(path string, c *Config) error {
    pandocConf := PandocConfig{
        Title: c.Meta.Title,
        Author: c.Meta.Author,
        DocumentClass: c.Style.DocumentClass,
        ClassOptions: c.Style.ClassOptions,
        NumberSections: c.Style.NumberSections,
        ReferencesTitle: c.Bib.ReferencesTitle,
        Csl: c.Bib.Csl,
        LinkCitations: c.Bib.LinkCitations,
    }

    // Handle some special values
    switch c.Meta.Date {
    case "today", "now", "present":
		pandocConf.Date = "\\today"
    default:
        pandocConf.Date = c.Meta.Date
    }

	switch c.Build.OutputFormat {
	case "html":
		pandocConf.HeaderIncludes = c.Html.Header
	case "pdf":
        if len(c.Latex.Packages) > 0 {
            for _, pack := range c.Latex.Packages {
                if string(pack[0]) == "[" || string(pack[0]) == "{" {
                    // Some extra options are specified
                    pandocConf.HeaderIncludes += "\\usepackage" + pack + "\n" 
                } else {
                    // Just a normal package name
                    pandocConf.HeaderIncludes += "\\usepackage{" + pack + "}\n" 
                }
            }
        }
		pandocConf.HeaderIncludes += c.Latex.Header
	}

	// Marshal config struct into JSON
	jsonBytes, err := json.Marshal(&pandocConf)
	if err != nil {
		return errors.New("Could not marshal metadata into JSON. " + err.Error())
	}

	// Write the JSON to path
	err = os.WriteFile(path, jsonBytes, 0644)
	if err != nil {
		return errors.New("Could not write JSON file. " + err.Error())
	}

	return nil
}
