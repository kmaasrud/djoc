package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"path"
	"time"
	"reflect"
)

type BibEntry struct {
	Id				string				`json:"id"`
	Type			string				`json:"type"`
	Title			string				`json:"title"`
	Publisher		string				`json:"publisher"`
	Accessed		time.Time			`json:"accessed"`
	Issued			time.Time			`json:"issued"`
	Author			[]struct {
		Family		string				`json:"family"`
		Given		string				`json:"given"`
		Literal		string				`json:"literal"`
		NonDropPart string				`json:"non-dropping-particle"`
		DropPart	string				`json:"dropping-particle"`
	}									`json:"author"`
	Fields			map[string]string	`json:"-"`
}

const zoteroBase string = "https://api.zotero.org"

func GetZoteroItem(query string, userId string) {
	// Create the request URL
	u, _ := url.Parse(zoteroBase)
	u.Path = path.Join(u.Path, "users", userId, "items")

	// Add the query. Fetch bibliography entry as CSL JSON
	q := u.Query()
	q.Add("v", "3")
	q.Add("q", query)
	q.Add("format", "csljson")
	u.RawQuery = q.Encode()

	// Fetch the response
	resp, err := http.Get(u.String())
	if err != nil {
		print(err)
	}
	defer resp.Body.Close()

	// Read into []byte
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		print(err)
	}

	// Unmarshal "items" field into a list of raw JSON to parse individually
	var results struct {
		Items []json.RawMessage
	}
	json.Unmarshal(body, &results)

	// Do parsing of each result
	var entries []BibEntry
	for _, entry := range results.Items {
		var e BibEntry

		// Unmarshal known fields
		json.Unmarshal(entry, &e)

		// Unmarshal all fields into map and delete known fields (essentially storing unknown fields)
		json.Unmarshal(entry, &e.Fields)
		delete(e.Fields, "id")
		delete(e.Fields, "type")
		delete(e.Fields, "title")
		delete(e.Fields, "author")

		// Process dates
		var dateStruct struct {
			Issued struct {
				DateParts [][]int	`json:"date-parts"`
			} `json:"issued"`
			Accessed struct {
				DateParts [][]int	`json:"date-parts"`
			} `json:"accessed"`
		}
		// This is extremely verbose, but it works
		if err := json.Unmarshal(entry, &dateStruct); err == nil {
			process := func(d [][]int) time.Time {
				if len(d[0]) > 2 {
					return time.Date(d[0][0], time.Month(d[0][1]), d[0][2], 0, 0, 0, 0, time.UTC)
				} else if len(d[0]) == 2 {
					return time.Date(d[0][0], time.Month(d[0][1]), 1, 0, 0, 0, 0, time.UTC)
				} else {
					return time.Date(d[0][0], time.Month(1), 1, 0, 0, 0, 0, time.UTC)
				}
			}
			if d := dateStruct.Issued; !reflect.ValueOf(d).IsZero() {
				// NOTE: Assuming "date-parts". It seems Zotero parses dates themselves, and does not use "raw"
				e.Issued = process(d.DateParts)
			}
			if d := dateStruct.Accessed; !reflect.ValueOf(d).IsZero() {
				e.Accessed = process(d.DateParts)
			}
		}

		entries = append(entries, e)
	}
	for _, entry := range entries {
		fmt.Printf("%+v\n", entry.Title)
		fmt.Printf("%+v\n", entry.Issued)
	}
}

func main() {
	GetZoteroItem("hartree", "7721787")
}
