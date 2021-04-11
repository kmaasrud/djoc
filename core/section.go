package core

import (
    "path/filepath"
    "strings"
    "strconv"
)

type Section struct {
    path string
    title string
    index int
}

func SectionFromPath(path string) (Section, error) {
    split := strings.Split(strings.TrimSuffix(filepath.Base(path), filepath.Ext(path)), "_")
    title := strings.Join(split[1:], "")
    index, err := strconv.Atoi(split[0])
    if err != nil {
        return Section{}, err
    }

    return Section{ path, title, index }, nil
}

func PathsFromSections(secs []Section) []string {
    var paths []string
    for _, sec := range secs {
        paths = append(paths, sec.path)
    }
    return paths
}
