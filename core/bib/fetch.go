package main

import (
	"net/url"
	"path"
	"net/http"
	"fmt"

	"github.com/nickng/bibtex"
)

const zoteroBase string = "https://api.zotero.org"

func GetZoteroItem(query string) {
	u, _ := url.Parse(zoteroBase)
	u.Path = path.Join(u.Path, "users", "7721787", "items")
	q := u.Query()
	q.Add("q", query)
	q.Add("format", "bibtex")
	u.RawQuery = q.Encode()
	fmt.Println(u.String())
	resp, err := http.Get(u.String())
	if err != nil {
		print(err)
	}
	defer resp.Body.Close()

	bib, err := bibtex.Parse(resp.Body)
	fmt.Println(bib)
}

func main() {
	GetZoteroItem("Blocking")
}
