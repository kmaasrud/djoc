package core

import (
	"fmt"
	"github.com/kmaasrud/doctor/msg"
	"os"
)

func createAt(path string) {
	var f *os.File
	var err error
	new_file := "./test2"
    if _, err := os.Stat(path); os.IsNotExist(err) {
		f, err = os.Create(new_file)
		f.WriteString("How are you dude?\n")
	} else {
		f, err = os.Open(new_file)
	}
	defer f.Close()
	if err != nil {
		msg.Error(err.Error())
	}

	data := make([]byte, 100)
	count, err := f.Read(data)
	if err != nil {
		msg.Error(err.Error())
	}

	if count > 1 {
		fmt.Printf("Data read: %s", data)
	}
}
