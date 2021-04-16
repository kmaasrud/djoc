package msg

import (
    "fmt"
    "strconv"
    "strings"

    "github.com/kmaasrud/doctor/core"
)

// Enters interactive mode to select among a choice of sections. The boolean returned is
// whether the user quit the interactive mode or not.
func ChooseSection(secs []core.Section, initMessage, choiceMessage string) (core.Section, bool) {
    var chosenIndex string
    var choice core.Section

    Info(initMessage)
    for true {
        for i, sec := range secs {
            fmt.Printf(" %d. %s\n", i+1, sec.Title)
        }
        fmt.Print(choiceMessage + " (q to quit) ")
        fmt.Scanln(&chosenIndex)
        index, err := strconv.Atoi(chosenIndex)
        if err == nil && index > 0 && index <= len(secs) {
            choice = secs[index-1]
            break
        } else if strings.ToLower(chosenIndex) == "q" {
            return choice, true
        } else {
            Info("That is not a valid option. Please enter the number of the section you want to remove.")
        }
    }
    return choice, false
}
