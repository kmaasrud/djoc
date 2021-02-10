# :page_facing_up: KODB - [kmaasrud's](https://github.com/kmaasrud) Opinionated Document Builder

This is my personal and (extremely) opinionated document builder script, built around my personal document creation workflow.

When doing scientific writing, I write using [pandoc](https://pandoc.org/) and it's flavor of Markdown. I absolutely adore Pandoc, because of it's immense flexibility and simplicity, that comes without losing the power of LaTeX (if I lack any features, I simply write some inline LaTeX, and it will be parsed perfectly). However, some kind of LaTeX engine is necessary, which usually defaults to the rather bloated and messy TeXLive :vomiting_face: To solve this, I use [Tectonic](https://tectonic-typesetting.github.io/en-US/), an excellent and modernized LaTeX engine built in C and Rust (it's much faster). I also need an intuitive Markdown syntax for cross-referencing and to handle bibliography. For this I have a couple of preferred Pandoc filters I regularly use.

In total, this leads to quite a convoluted process, and to keep my head clear, the project requires a strict and clean directory structure. KODB automates everything for me, and ensures a consistent structure and workflow for my scientific writing. This also facilitates easier collaboration.

## Installation

To run KODB, a working Python installation, including `pip` must be available. Install by running

    sudo python -m pip install git+https://github.com/kmaasrud/kodb

If you have both Python 2 and 3 installed, be sure to run the above command with `python3`.

In addition, [Tectonic](https://tectonic-typesetting.github.io/en-US/index.html) and [Pandoc](https://pandoc.org/) need to be installed. On Debian-based systems, this can be done with these two commands (assuming a working rustup-installation is set up):

    sudo apt-get install libfontconfig1-dev libgraphite2-dev libharfbuzz-dev libicu-dev libssl-dev pandoc zlib1g-dev

    cargo install tectonic

Instructions for installing on other platforms is found on their respective websites.

## Short tutorial

- You start a document inside an existing directory by running `kodb init` or in a new directory by running `kodb new <project name>`.
- Adding a new section to the document is as easy as running `kodb add <section name>`.
- When you're ready to build the PDF, run `kodb build`.

The document sections are located in the `src` directory, and are written in Pandoc Markdown. They support cross-referencing through [pandoc-xnos](https://github.com/tomduck/pandoc-xnos). As a ground rule, do not rename the Markdown-files, as this may cause issues. Assets (pictures, figures and similar), should be placed in the `assets` directory.

The metadata is described in the `kodb.yaml` file located in the root directory. Here the title of the document, the author(s), the date and more can be declared.

