package lua

import _ "embed"

//go:embed crossref.lua
var crossref []byte

//go:embed abstract.lua
var abstract []byte

var Filters = map[string][]byte{
	".crossref.lua": crossref,
	".abstract.lua": abstract,
}
