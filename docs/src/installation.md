---
title: Install Doctor
toc: false
---

# Installation

To download Doctor, either take a look at one of the pre-compiled binaries on [GitHub](https://github.com/kmaasrud/doctor/releases){target="_blank" rel="noopener noreferrer"}, or download it with the [Go tool](https://golang.org/doc/install){target="_blank" rel="noopener noreferrer"} by running:

	go install github.com/kmaasrud/doctor

If you are on a Unix system and have Bash installed, you can install both Go and Doctor automatically by running this shell script:

	wget -q -O - https://www.kmaasrud.com/doctor/install.sh | bash

or, if you do not have `wget`, use `curl`.

	curl https://www.kmaasrud.com/doctor/install.sh | bash

## Dependencies

Doctor needs one crucial component in order to work correctly, namely [Pandoc](https://pandoc.org/){target="_blank" rel="noopener noreferrer"}. Have a look at [Installing Pandoc](https://pandoc.org/installing.html){target="_blank" rel="noopener noreferrer"} and make sure it is available in your PATH by running `pandoc -v`.

Additionally, if you want to produce PDFs, you will need a $\TeX$ distribution containing a PDF engine. The fastest alternative is [TeXLive's](https://tug.org/texlive/){target="_blank" rel="noopener noreferrer"} `pdflatex`, but beware that it does not support Unicode. TeXLive also contains `lualatex` and `xelatex`, both of which support scripting, better font coverage and full Unicode. [MikTeX](https://miktex.org/){target="_blank" rel="noopener noreferrer"} is also a great distribution, containing all of the aforementioned engines. The lightest alternative is probably [Tectonic](https://tectonic-typesetting.github.io/){target="_blank" rel="noopener noreferrer"}. It is almost as fast as `pdflatex` and automatically downloads packages for you, which means you can avoid the multiple gigabyte install of TeXLive. Beware that this is beta software, and not as stable as other alternatives. 

With all this covered, you are ready to write your first document! A "Getting started guide" will soon be available to get you flying, but for now you can just have a look at the [documentation](docs) to figure out how Doctor works.
