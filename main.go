package main

import (
    "time"
	"github.com/kmaasrud/kodb/msg"
)

func main() {
    done := make(chan struct{})
    go thisTakesTime(done)
    msg.Do("Doing stuff, wait a bit", "The stuff is done!", done)
}

func thisTakesTime(done chan struct{}) {
    time.Sleep(5 * time.Second)
    close(done)
}
