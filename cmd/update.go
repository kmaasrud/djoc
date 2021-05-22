package cmd

import (
	"errors"
    "fmt"
    "strings"
    "os"

	"github.com/equinox-io/equinox"
    "github.com/blang/semver"
    "github.com/rhysd/go-github-selfupdate/selfupdate"
	"github.com/kmaasrud/doctor/msg"
)

func Update(ver string) error {
    if ver == "DEV" {
        msg.Info("You are on the DEV version (downloaded with the Go tool.)\n    Run 'go install github.com/kmaasrud/doctor@latest' to update.")
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
        return errors.New("Error occurred while updating binary: "  + err.Error())
    }

    msg.Success("Successfully updated to version " + latest.Version.String())
    return nil
}

// Below is for Equinox
const appID = "app_gvefXKeSXD5"

var publicKey = []byte(`
-----BEGIN ECDSA PUBLIC KEY-----
MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAEBIokTYcFzVBGV68Vs+32HiIFdIyRfUeZ
ggZtn72eXWLSzARQCtDtC05lAWu/7DZj1kpkC5aX1iiZ0Luw4135nHNXGcTch0/f
EnlrZMZSJhNdxu2/9VhgG/UEISHrp0iX
-----END ECDSA PUBLIC KEY-----
`)

func EquinoxUpdate() error {
	done := make(chan struct{})
	go msg.Do("Looking for new version...", done)
	var opts equinox.Options
	if err := opts.SetPublicKeyPEM(publicKey); err != nil {
		msg.CloseDo(done)
		return errors.New("Could not set public key. " + err.Error())
	}

	// check for the update
	resp, err := equinox.Check(appID, opts)
	msg.CloseDo(done)
	switch {
	case err == equinox.NotAvailableErr:
		msg.Info("No update available, already at the latest version!")
		return nil
	case err != nil:
		return errors.New("Update failed: " + err.Error())
	}

	// fetch the update and apply it
	done = make(chan struct{})
	go msg.Do("Found update! Applying it...", done)
	err = resp.Apply()
	msg.CloseDo(done)
	if err != nil {
		return errors.New("Could not apply update. " + err.Error())
	}

	msg.Success("Updated to new version: " + resp.ReleaseVersion + "!")
	return nil
}
