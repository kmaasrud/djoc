package main

import (
    "os"
)

func check_err(e error) {
    if e != nil {
        panic(e)
    }
}

func path_exists(path string) bool {
    _, err := os.Stat(path)
    return !os.IsNotExist(err)
}
