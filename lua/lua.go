package lua

import _ "embed"

//go:embed crossref.lua
var crossref []byte

var Filters = map[string][]byte{".crossref.lua": crossref}
