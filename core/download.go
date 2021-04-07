package core

import (
    "os"
    "io"
    "os/exec"
    "errors"
    "path/filepath"
    "net/http"
    "archive/tar"
    "compress/gzip"

    "github.com/kmaasrud/doctor/msg"
)

// Checks if the dependencies of Doctor are present.
func CheckDependencies() error {
    deps := map[string]string{"pandoc": "2.13", "tectonic": "0.4.1"}
    home, err := os.UserHomeDir()
    if err != nil {
        msg.Error(err.Error())
    }

    // This is Unix specific. TODO: Find paths for Windows too
    doctorPath := filepath.Join(home, ".local", "share", "doctor")

	for dep, ver := range deps {
        if _, err := os.Stat(filepath.Join(doctorPath, dep + "-" + ver, "bin", dep)); err == nil {
            continue
        }
		_, err := exec.LookPath(dep)
		if err != nil {
            if dep == "pandoc" {
                err := downloadPandoc(filepath.Join(doctorPath), ver)
                if err != nil {
                    msg.Error("Could not download Pandoc: " + err.Error())
                }
            } else {
                return errors.New("Could not find " + msg.Style(dep, "Bold") + " in your PATH.")
            }
	    }
    }
	return nil
}

func downloadPandoc(dlDir string, version string) error {
    done := make(chan struct{})

    go msg.Do("Downloading Pandoc tarball", done)
    url := "https://github.com/jgm/pandoc/releases/download/" + version + "/pandoc-" + version + "-linux-amd64.tar.gz"
    resp, err := http.Get(url)
    if err != nil {
        msg.CloseDo(done)
        msg.Error(err.Error())
        os.Exit(1)
    }
    defer resp.Body.Close()

    // Check if dlDir exists, else make it
	if _, existErr := os.Stat(dlDir); os.IsNotExist(existErr) {
        msg.Info("Could not find local Doctor storage directory, making it...")
		err := os.Mkdir(dlDir, 0777)
		if err != nil {
			return errors.New("Could not create Doctor local storage directory: " + err.Error())
		}
    }
    f, err := os.Create(filepath.Join(dlDir, "pandoc.tar.gz"))
    if err != nil {
        msg.CloseDo(done)
        msg.Error(err.Error())
        os.Exit(1)
    }

    _, err = io.Copy(f, resp.Body)
    if err != nil {
        msg.CloseDo(done)
        msg.Error(err.Error())
        os.Exit(1)
    }
    msg.CloseDo(done)
    msg.Success("Pandoc tarball downloaded.")

    f.Seek(0, 0)
    gzr, err := gzip.NewReader(f)
	if err != nil {
        msg.Debug("You're here")
        return err
	}
	defer gzr.Close()

    done = make(chan struct{})
    go msg.Do("Untarring Pandoc tarball", done)
    tr := tar.NewReader(gzr)
    for {
		header, err := tr.Next()

		switch {
		// If no more files are found, mark as done and return
		case err == io.EOF:
            msg.CloseDo(done)
            msg.Success("Pandoc untarred into " + filepath.Join(dlDir, "pandoc-" + version))
			return nil
		// Return any other error
		case err != nil:
            msg.CloseDo(done)
			return err
		// If the header is nil, just skip it (not sure how this happens)
		case header == nil:
			continue
		}

		// The target location where the dir/file should be created
		target := filepath.Join(dlDir, header.Name)

		// Check the file type
		switch header.Typeflag {

		// If its a dir and it doesn't exist create it
		case tar.TypeDir:
			if _, err := os.Stat(target); err != nil {
				if err := os.MkdirAll(target, 0755); err != nil {
                    msg.CloseDo(done)
					return err
				}
			}

		// If it's a file create it
		case tar.TypeReg:
			f, err := os.OpenFile(target, os.O_CREATE|os.O_RDWR, os.FileMode(header.Mode))
			if err != nil {
                msg.CloseDo(done)
				return err
			}

			// Copy over contents
			if _, err := io.Copy(f, tr); err != nil {
                msg.CloseDo(done)
				return err
			}
			
			// Manually close here after each file operation; defering would cause each file close
			// to wait until all operations have completed.
			f.Close()
		}
	}
}
