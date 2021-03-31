package main

import (
    "fmt"
    "os"
    "github.com/kmaasrud/kodb/utils"
)

func createAt(path string) {
	var f *os.File
	var err error
	new_file := "./test2"
	if !utils.PathExists(new_file) {
		f, err = os.Create(new_file)
		f.WriteString("How are you dude?\n")
	} else {
		f, err = os.Open(new_file)
	}
	defer f.Close()

	utils.CheckErr(err)
	data := make([]byte, 100)
	count, err2 := f.Read(data); utils.CheckErr(err2)
	if count > 1 {
		fmt.Printf("Data read: %s", data)
	}
}
