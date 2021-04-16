package core

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// The string separating the index and the name. If changed, make a due notice to users and
// either ensure backwards compatibility or have Doctor change the format automatically.
const SectionSep string = "_"

// Represents a section in the document.
type Section struct {
	Path  string
	Title string
	Index int
}

// Check whether this section is equal to another. Checks if their paths are equal.
func (s Section) IsEqual(other Section) bool {
	return s.Path == other.Path
}

// Changes the index of this section by renaming the file it represents.
func (s *Section) ChangeIndex(i int) error {
	s.Index = i
	newFilename := fmt.Sprintf("%02d_", i) + strings.Join(strings.Split(filepath.Base(s.Path), SectionSep)[1:], "")
	newPath := filepath.Join(filepath.Dir(s.Path), newFilename)

	err := os.Rename(s.Path, newPath)
	if err != nil {
		return err
	}
	s.Path = newPath
	return nil
}

// Creates a new Section struct from the input path.
func SectionFromPath(path string) (Section, error) {
	split := strings.Split(strings.TrimSuffix(filepath.Base(path), filepath.Ext(path)), SectionSep)
	title := strings.Join(split[1:], "") // TODO: Find title from header of file
	index, err := strconv.Atoi(split[0])
	if err != nil {
		return Section{}, err
	}

	return Section{path, title, index}, nil
}

// Takes a list of Section structs and outputs a list of the corresponding paths.
func PathsFromSections(secs []Section) []string {
	var paths []string
	for _, sec := range secs {
		paths = append(paths, sec.Path)
	}
	return paths
}
