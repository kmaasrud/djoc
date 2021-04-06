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
	deps := [2]string{"pandoc", "tectonic"}
    home, err := os.UserHomeDir()
    if err != nil {
        msg.Error(err.Error())
    }

    // This is Unix specific. TODO: Find paths for Windows too
    doctorPath := filepath.Join(home, ".local", "share", "doctor")

	for _, dep := range deps {
        if _, err := os.Stat(filepath.Join(doctorPath, dep)); err == nil {
            continue
        }
		_, err := exec.LookPath(dep)
		if err != nil {
            if dep == "pandoc" {
                done := make(chan struct{})
                err := downloadPandoc(filepath.Join(doctorPath, "pandoc"), "2.13", done)
                msg.Do("Could not find Pandoc in PATH, downloading it locally", "Downloaded Pandoc into " + msg.Style(doctorPath, "Bold"), done)
            } else {
                return errors.New("Could not find " + msg.Style(dep, "Bold") + " in your PATH.")
            }
	    }
    }
	return nil
}

func downloadPandoc(dlDir string, version string) error {
    done := make(chan struct{})
    errCh := make(chan error)
    fCh := make(chan *os.File)
    defer close(fCh)
    go func() {
        url := "https://github.com/jgm/pandoc/releases/download/" + version + "/pandoc-" + version + "-linux-amd64.tar.gz"
        resp, err := http.Get(url)
        if err != nil {
            msg.Error(err.Error())
        }
        defer resp.Body.Close()

        f, err := os.Create(filepath.Join(dlDir, "pandoc.tar.gz"))
        if err != nil {
            errCh <- err
            return
        }

        _, err = io.Copy(f, resp.Body)
        if err != nil {
            errCh <- err
            return
        }
        fCh <- f
        close(done)
    }()
    msg.Do("Downloading Pandoc tarball", "Pandoc tarball downloaded.", done)

    gzr, err := gzip.NewReader(<- fCh)
	if err != nil {
        return err
	}
	defer gzr.Close()

    tr := tar.NewReader(gzr)
    for {
		header, err := tr.Next()

		switch {
		// If no more files are found, mark as done and return
		case err == io.EOF:
            close(done)
			return nil
		// Return any other error
		case err != nil:
			return err
		// If the header is nil, just skip it (not sure how this happens)
		case header == nil:
			continue
		}

		// the target location where the dir/file should be created
		target := filepath.Join(dlDir, header.Name)

		// the following switch could also be done using fi.Mode(), not sure if there
		// a benefit of using one vs. the other.
		// fi := header.FileInfo()

		// check the file type
		switch header.Typeflag {

		// if its a dir and it doesn't exist create it
		case tar.TypeDir:
			if _, err := os.Stat(target); err != nil {
				if err := os.MkdirAll(target, 0755); err != nil {
					return err
				}
			}

		// if it's a file create it
		case tar.TypeReg:
			f, err := os.OpenFile(target, os.O_CREATE|os.O_RDWR, os.FileMode(header.Mode))
			if err != nil {
				return err
			}

			// copy over contents
			if _, err := io.Copy(f, tr); err != nil {
				return err
			}
			
			// manually close here after each file operation; defering would cause each file close
			// to wait until all operations have completed.
			f.Close()
		}
	}
}
