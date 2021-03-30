package main

import (
    "fmt"
    "os"
)

func main() {
	var f *os.File
	var err error
	new_file := "./test2"
	if !path_exists(new_file) {
		f, err = os.Create(new_file)
		f.WriteString("How are you dude?\n")
	} else {
		f, err = os.Open(new_file)
	}
	defer f.Close()

	check_err(err)
	data := make([]byte, 100)
	count, err2 := f.Read(data); check_err(err2)
	if count > 1 {
		fmt.Printf("Data read: %s", data)
	}
}
