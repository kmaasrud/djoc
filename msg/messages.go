package msg

import (
    "fmt"
    "time"
    "strings"
)

func Error(text string) {
    fmt.Printf("%s%s%s %s\n", Style("[", "Gray"), Style("E", "Red", "Bold"), Style("]:", "Gray"), text)
}

func Info(text string) {
    fmt.Printf("%s %s\n", Style("[I]:", "Gray"), text)
}

func Success(text string) {
    fmt.Printf("%s%s%s %s\n", Style("[", "Gray"), Style("✓", "Green", "Bold"), Style("]:", "Gray"), text)
}

func Debug(text string) {
    fmt.Printf("%s %s\n", Style("[D]:", "Gray"), text)
}

func Do(doingText , doneText string, done chan struct{}) {
    ticker := time.NewTicker(500 * time.Millisecond)
    defer fmt.Printf("\033[2K\r%s%s%s %s\n", Style("[", "Gray"), Style("✓", "Green", "Bold"), Style("]:", "Gray"), doneText)
    defer ticker.Stop()
    for i := 0;; {
        select {
            case <-ticker.C:
                i = i % 3
                dots := strings.Repeat(".", i+1)
                fmt.Printf("\033[2K\r%s %s%s", Style("[*]:", "Gray"), doingText, dots)
                i += 1
            case <-done:
                return
        }
    }
}
