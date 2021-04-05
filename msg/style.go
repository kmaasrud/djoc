package msg

import "fmt"

// style takes the inputted text and styles it according to
// the ANSI escape codes listed below. I should perhaps check for
// non-ANSI systems, but fuck that for now...
func Style(text string, styles ...string) string {
    code := map[string]int {
        "Red": 31,
        "Green": 32,
        "Yellow": 33,
        "Blue": 34,
        "Magenta": 35,
        "Cyan": 36,
        "BrightRed": 91,
        "BrightGreen": 92,
        "BrightYellow": 93,
        "BrightBlue": 94,
        "BrightMagenta": 95,
        "BrightCyan": 96,
        "Bold": 1,
        "Faint": 2,
        "Italic": 3,
        "Underline": 4,
        "Blink": 5,
        "Strike": 9,
    }

    for _, style := range styles {
        text = fmt.Sprintf("\033[%vm%v\033[0m", code[style], text)
    }

    return text
}
