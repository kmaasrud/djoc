package msg

import (
	"fmt"
	"os"
	"strings"
	"time"
)

func Error(text string) {
	fmt.Fprintln(os.Stderr, fmt.Sprintf(" %s  %s", Style("E", "Red", "Bold"), text))
}

func Warning(text string) {
	fmt.Fprintln(os.Stderr, fmt.Sprintf(" %s  %s", Style("W", "Yellow", "Bold"), text))
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
// of stderr and allows me to filter away lines I don't want. It also splits them into errors
// and warnings, allowing me to separate them and style them.
func CleanStderrMsg(stderr string) (string, string) {
	var warnings, errors string
	for _, line := range strings.Split(strings.TrimSuffix(stderr, "\n"), "\n") {
		if line == "" {
			continue
		}

		if strings.HasPrefix(line, "! ") {
			errors += "        " + Style("TeX: ", "Bold") + strings.TrimPrefix(line, "! ") + "\n"
		} else if strings.HasPrefix(line, "[WARNING] ") {
			warnings += "        " + Style("Pandoc: ", "Bold") + strings.TrimPrefix(line, "[WARNING] ") + "\n"
		} else if strings.HasPrefix(line, "[ERROR] ") {
			errors += "        " + Style("Pandoc: ", "Bold") + strings.TrimPrefix(line, "[ERROR] ") + "\n"
		} else {
			errors += "        " + line + "\n"
		}
	}
	return warnings, errors
}
