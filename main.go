package main

import (
	"github.com/kmaasrud/doctor/msg"
	"github.com/kmaasrud/doctor/utils"
	"time"
	"fmt"
	"os"
)

func main() {
	/* done := make(chan struct{})
	go thisTakesTime(done)
	msg.Do("Doing stuff, wait a bit", "The stuff is done!", done) */
	root, err := utils.FindDoctorRoot()
	if err != nil {
		msg.Error(err.Error())
		os.Exit(1)
	}
	fmt.Println(root)
}

func thisTakesTime(done chan struct{}) {
	time.Sleep(5 * time.Second)
	close(done)
}
