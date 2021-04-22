---
title: Installing Doctor
toc: false
---

The latest stable release of Doctor is available on the [**download page**](https://dl.equinox.io/kmaasrud/doctor/stable){target="_blank" rel="noopener noreferrer"}. If you are on a Unix system with Bash installed, there's a short shell script that automatically downloads the binary locally and adds it to your `PATH`. Just run

```shell
curl https://www.kmaasrud.com/doctor/install.sh | bash
```

If you have the [Go tool](https://golang.org/doc/install){target="_blank" rel="noopener noreferrer"} installed, you can alternatively download Doctor by running:

```shell
go install github.com/kmaasrud/doctor@latest
```

# Dependencies

Doctor needs one crucial component in order to work correctly, namely [Pandoc](https://pandoc.org/){target="_blank" rel="noopener noreferrer"}. Have a look at [Installing Pandoc](https://pandoc.org/installing.html){target="_blank" rel="noopener noreferrer"} and make sure it is available in your `PATH` by running `pandoc -v`.


## PDF creation with a $\TeX$ distribution

If you want to produce PDFs, you will need a $\TeX$ distribution containing a PDF engine. The fastest alternative is [TeXLive's](https://tug.org/texlive/){target="_blank" rel="noopener noreferrer"} `pdflatex`, but beware that it does not fully support Unicode. In cases where this is needed, TeXLive also contains `lualatex` and `xelatex`, both of which have better font coverage and Unicode support. Refer to the [TeXLive homepage](https://tug.org/texlive/){target="_blank" rel="noopener noreferrer"} for how to install this distribution on your system.

[MiKTeX](https://miktex.org/){target="_blank" rel="noopener noreferrer"} is also a great alternative, containing all of the aforementioned engines. In my testing MiKTeX was quite a lot slower than TeXLive, but it comes with the advantage of auto-installing missing packages while compiling. This means you can get away with a smaller install size and avoid the dreaded dependency hell many face when using TeXLive.

That leaves me with the lightest alternative, which is self-contained, reasonably fast and auto-installs missing packages, namely [Tectonic](https://tectonic-typesetting.github.io/){target="_blank" rel="noopener noreferrer"}. For me it has proven to be the best compromise between speed and minimal bloat, which makes it a perfect alternative for those who do not already have a $\TeX$ distribution on their system. Beware that this is beta software, and thus not as stable as other alternatives.

Be sure that the engine you choose is in your `PATH` (for example by running `pdflatex` to see if you get an error) and edit the `engine` option in `doctor.toml` to use your preffered engine. See also [the configuration docs](config#engine) for more info.

# Updating

To update Doctor to a new version, simply run

```shell
doctor --update
```

This will check for an update, download it if it exists and replace the old binary with the new one.

> **Note**: If the Doctor binary is placed in a directory requiring administrator access, so too will the `doctor --update` command. This means you'll have to run it like `sudo doctor --update` or similar.

---

With all this covered, you are ready to write your first document! See [Creating your first document](creating-your-first-document) for a quick guide on how to get started.
