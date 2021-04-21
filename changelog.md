# Changelogs

## Doctor v0.2.0 (2020-04-21)

Doctor is now published to Equinox. This means it has multiple installers and supports self-updating. You can replace your previous binary or Go install with the new one on the [download page](https://dl.equinox.io/kmaasrud/doctor/stable), and update to any new version with `doctor --update`.

### New features

- [x] Command: `doctor move` to change the position of sections.
- [x] Command: `doctor list` to list the sections of your document, along with their index.
- [x] Command: `doctor --update` to update Doctor to the newest version.

### Improvements

- [x] Added options to `doctor.toml`:
    - [x] Boolean option: Allow turning off all Lua filters with the `lua-filters`.
    - [x] Boolean option: Specify if you want numbered sections or not with `number-sections`.
    - [x] String option: Specify an inclusion into the LaTeX header with `latex-header`.
        
        > This option only has an effect when the output format is `pdf`. For now that is the only option, so you won't have to worry about this just yet.

    - [x] String option: Specify a title for the section containing your references with the `references-title` option.
- [x] Even though it is mostly unused for now, the Doctor data dir now respects `$XDG_DATA_DIR` and `%APPDATA%`.
- [x] Big revamp to documentation and lots of stuff added to it.

### Bug fixes

- [x] Important error messages should no longer be swallowed by the Doctor messaging system.
- [x] Directory `src` was added as a resource path. This is now changed to `secs`.
- [x] Doctor forced you to have Tectonic installed to properly function. This is no longer the case.

## Doctor v0.1.2 (2020-04-15)

-  Added versioning feature. Run `doctor --version` or `doctor -v` to see which version you are currently on.

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
