package core

import (
    "path/filepath"
    "strings"
    "strconv"
    "fmt"
)

var SectionSep string = "_"

type Section struct {
    Path string
    Title string
    Index int
}

func (s *Section) ChangeIndex(i int) {
    s.Index = i
    newFilename := fmt.Sprintf("%02d_", i) + strings.Join(strings.Split(filepath.Base(s.Path), SectionSep)[1:], "")
    s.Path = filepath.Join(filepath.Dir(s.Path), newFilename)
}

func SectionFromPath(path string) (Section, error) {
    split := strings.Split(strings.TrimSuffix(filepath.Base(path), filepath.Ext(path)), SectionSep)
    title := strings.Join(split[1:], "") // TODO: Find title from header of file
    index, err := strconv.Atoi(split[0])
    if err != nil {
        return Section{}, err
    }

    return Section{ path, title, index }, nil
}

func PathsFromSections(secs []Section) []string {
    var paths []string
    for _, sec := range secs {
        paths = append(paths, sec.Path)
    }
    return paths
}
