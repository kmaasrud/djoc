package core

import (
	"github.com/kmaasrud/doctor/msg"
	"os"
    "fmt"
)

func CreateAt(path string) {
    if _, existErr := os.Stat(path); os.IsNotExist(existErr) {
        err := os.Mkdir(path, 0777)
        if err != nil {
            msg.Error(err.Error())
        }
	} else {
        msg.Error(fmt.Sprintf("The directory %s already exists", msg.Style(path, "Bold")))
	}
}
