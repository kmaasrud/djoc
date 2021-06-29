package utils

import (
    "os/exec"
    "bytes"
)

type WarningError struct {
	Stderr string
}

func (e *WarningError) Error() string {
	return e.Stderr
}

type FatalError struct {
	Stderr string
}

func (e *FatalError) Error() string {
	return e.Stderr
}

func RunPandocWith(args []string) (string, error) {
    var stdout bytes.Buffer
	var stderr bytes.Buffer
	cmd := exec.Command("pandoc", args...)
    cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()

    out := string(stdout.Bytes())

	// Fatal error
	if err != nil {
		return out, &FatalError{string(stderr.Bytes())}
	}

	// Non-fatal, but stderr is not empty, so it includes warnings
	if stderr := string(stderr.Bytes()); len(stderr) != 0 {
		return out, &WarningError{string(stderr)}
	}
	return out, nil
}
