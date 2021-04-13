# Changelogs for Doctor

## Doctor v0.1.0 (Unknown release date)

KODB is now rebranded as Doctor and written totally from scratch in Go. Say hello to a document builder that is blazingly fast, safe and most importantly a pleasure to use.

- [x] `doctor` command line tool that parses commands supplied to it.
- [x] Command: `doctor new` for creating a new document.
- [x] Command: `doctor build` for building a PDF from the Doctor document (currently only supporting [Tectonic](https://tectonic-typesetting.github.io/))
- [x] Command: `doctor add` for adding sections.
- [x] Command: `doctor remove` for removing sections.
- [ ] Smashing new documentation hosted on it's own webpage.
- [ ] TOML file for configuring attributes and compile options.
