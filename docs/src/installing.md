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

Additionally, if you want to produce PDFs, you will need a $\TeX$ distribution containing a PDF engine. The fastest alternative is [TeXLive's](https://tug.org/texlive/){target="_blank" rel="noopener noreferrer"} `pdflatex`, but beware that it does not support Unicode. TeXLive also contains `lualatex` and `xelatex`, both of which support scripting, better font coverage and full Unicode. [MikTeX](https://miktex.org/){target="_blank" rel="noopener noreferrer"} is also a great distribution, containing all of the aforementioned engines. The lightest alternative is probably [Tectonic](https://tectonic-typesetting.github.io/){target="_blank" rel="noopener noreferrer"}. It is almost as fast as `pdflatex` and automatically downloads packages for you, which means you can avoid the multiple gigabyte install of TeXLive. Beware that this is beta software, and not as stable as other alternatives. 

# Updating

To update Doctor to a new version, simply run

```shell
doctor --update
```

This will check for an update, download it if it exists and replace the old binary with the new one.

> **Note**: If the Doctor binary is placed in a directory requiring administrator access, so too will the `doctor --update` command. This means you'll have to run it like `sudo doctor --update` or similar.

---

With all this covered, you are ready to write your first document! See [Creating your first document](creating-your-first-document) for a quick guide on how to get started.
