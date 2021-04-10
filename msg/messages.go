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

// Tectonic, TeX and even Pandoc produce A LOT of noise. This function runs through each line
// of stderr and returns only those containing relevant information. This cleans up a lot and
// allows me to style the errors/warnings according to Doctor messages. I admit it might be a bit
// stupid, since I can never be sure to catch everything, but I think it is worth the debug time.
func CleanStderrMsg(stderr string) {
	includeNext := false
	for _, line := range strings.Split(strings.TrimSuffix(stderr, "\n"), "\n") {
		if includeNext {
			fmt.Println("         " + line)
			includeNext = false
		} else if strings.HasPrefix(line, "! ") {
			Error(Style("TeX: ", "Bold") + strings.TrimPrefix(line, "! "))
			includeNext = true
		} else if strings.HasPrefix(line, "error: ") {
			Error(Style("Tectonic: ", "Bold") + strings.TrimPrefix(line, "error: "))
		} else if strings.HasPrefix(line, "[WARNING] ") {
			Warning(Style("Pandoc: ", "Bold") + strings.TrimPrefix(line, "[WARNING] "))
		} else if strings.HasPrefix(line, "[ERROR] ") {
			Error(Style("Pandoc: ", "Bold") + strings.TrimPrefix(line, "[ERROR] "))
		}
	}
}
