package update

import (
	"errors"
	"fmt"
	"os"
	"strings"

	"github.com/blang/semver"
	"github.com/kmaasrud/doctor/msg"
	"github.com/rhysd/go-github-selfupdate/selfupdate"
)

func Update(ver string) error {
	if ver == "DEV" {
		fmt.Println("You are on the DEV version of Doctor (downloaded with the Go tool.)\nRun 'go install github.com/kmaasrud/doctor@latest' to update.")
		return nil
	}

	latest, found, err := selfupdate.DetectLatest("kmaasrud/doctor")
	if err != nil {
		return errors.New("Error occurred while detecting version: " + err.Error())
	}

	v := semver.MustParse(ver[1:])
	if !found || latest.Version.LTE(v) {
		msg.Success("Current version is the latest!")
		return nil
	}

	var confirmString string
	fmt.Printf("Do you want to update to version %s? (Y/n) ", msg.Style(latest.Version.String(), "Bold"))
	fmt.Scanln(&confirmString)
	if strings.ToLower(confirmString) == "n" {
		msg.Info("Keeping current version.")
		return nil
	}

	exe, err := os.Executable()
	if err != nil {
		return errors.New("Could not locate executable path")
	}

	if err := selfupdate.UpdateTo(latest.AssetURL, exe); err != nil {
		return errors.New("Error occurred while updating binary: " + err.Error())
	}

	msg.Success("Successfully updated to version " + latest.Version.String())
	return nil
}
