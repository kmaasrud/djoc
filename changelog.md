# Changelogs for Doctor

## Doctor v0.1.1 (2020-04-14)

Fixes issue with a malformed file `'` being part of the repo. You should now be able to install with

    go install github.com/kmaasrud/doctor@latest

Other than that, the binaries are exactly the same as in `v0.1.0`

## Doctor v0.1.0 (2020-04-13)

KODB is now rebranded as Doctor and written totally from scratch in Go. Say hello to a document builder that is blazingly fast, safe and most importantly a pleasure to use.

- [x] `doctor` command line tool that parses commands supplied to it.
- [x] Command: `doctor new` for creating a new document.
- [x] Command: `doctor build` for building a PDF from the Doctor document (currently only supporting [Tectonic](https://tectonic-typesetting.github.io/))
- [x] Command: `doctor add` for adding sections.
- [x] Command: `doctor remove` for removing sections.
- [x] Smashing new documentation hosted on it's own webpage.
- [x] TOML file for configuring attributes and compile options.
