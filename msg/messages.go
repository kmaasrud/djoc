package msg

import (
	"fmt"
	"strings"
	"time"
)

func Error(text string) {
	println(fmt.Sprintf(" %s  %s", Style("E", "Red", "Bold"), text))
}

func Warning(text string) {
	println(fmt.Sprintf(" %s  %s", Style("W", "Yellow", "Bold"), text))
}

func Info(text string) {
	fmt.Printf("    %s\n", Style(text, "Gray"))
}

func Success(text string) {
	fmt.Printf(" %s  %s\n", Style("✓", "Green", "Bold"), text)
}

func Debug(text string) {
	fmt.Printf("%s %s\n", Style("DEBUG:", "Gray"), text)
}

func Do(doingText string, done chan struct{}) {
	ticker := time.NewTicker(500 * time.Millisecond)
	defer ticker.Stop()
	for i := 0; ; {
		select {
		case <-ticker.C:
			i = i % 3
			dots := strings.Repeat(".", i+1) + strings.Repeat(" ", 2-i)
			fmt.Printf("\033[2K\r%s %s", Style(dots, "Gray"), doingText)
			i += 1
		case <-done:
			return 
		}
	}
}

func CloseDo(done chan struct{}) {
    close(done)
    fmt.Printf("\033[2K\r")
}
