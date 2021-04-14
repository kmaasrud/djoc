# Changelogs

## Doctor v0.2.0 (Unreleased)

### New features

- [ ] Command: `doctor move` to change the position of sections
- [ ] Command: `doctor list` to list the sections of your document, along with their index
- [ ] Command: `doctor edit` to open a section directly in your preferred text editor.

### Improvements

- [ ] Include custom content in the header of your LaTeX document with the `latex-header` option in `doctor.toml`
- [ ] Allow turning cross-referencing and other Lua filters on or off with the `cross-referencing` and `abstract-parsing` option in `doctor.toml`

### Bug fixes

- [ ] Important error messages should no longer be swallowed by the Doctor messaging system.

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
