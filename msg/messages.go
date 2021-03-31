package msg

import (
    "fmt"
    "time"
    "strings"
)

func Error(text string) {
    fmt.Printf("[%s]: %s\n", style("E", "Red", "Bold"), text)
}

func Info(text string) {
    fmt.Printf("[%s]: %s\n", style("I", "Blue", "Bold"), text)
}

func Do(doingText , doneText string, done chan struct{}) {
    ticker := time.NewTicker(500 * time.Millisecond)
    defer fmt.Printf("\033[2K\r[%s]: %s\n", style("✓", "Green", "Bold"), doneText)
    defer ticker.Stop()
    for i := 0;; {
        select {
            case <-ticker.C:
                i = i % 3
                dots := strings.Repeat(".", i+1)
                fmt.Printf("\033[2K\r[*]: %s%s", doingText, dots)
                i += 1
            case <-done:
                return
        }
    }
}

func Success(text string) {
    fmt.Printf("[%s]: %s\n", style("✓", "Green", "Bold"), text)
}
