---
title: Doctor documentation
---

Welcome to the Doctor documentation! Here you will find everything you need to know about writing documents with Doctor.

# Installation

To download Doctor, either take a look at one of the pre-compiled binaries on [GitHub](/){target="_blank" rel="noopener noreferrer"}, or download it with the [Go tool](https://golang.org/doc/install){target="_blank" rel="noopener noreferrer"} by running

	go install github.com/kmaasrud/doctor

Doctor needs one crucial component in order to work correctly, namely [Pandoc](https://pandoc.org/){target="_blank" rel="noopener noreferrer"}. Pandoc is actually so important that Doctor automatically downloads it for you if it is not already present in your system. Don't worry, if you have it installed already, you will not get a double.

Additionally, if you want to produce PDFs, you will need a $\TeX$ distribution containing a PDF engine. The fastest alternative is [TeXLive's](https://tug.org/texlive/){target="_blank" rel="noopener noreferrer"} `pdflatex`, which you probably already have installed. It also contains `luatex`, which sports scripting and better font support. A lighter alternative is [Tectonic](https://tectonic-typesetting.github.io/){target="_blank" rel="noopener noreferrer"}. It is almost as fast and automatically downloads packages for you, which means you can avoid the multiple gigabyte install of TeXLive. Beware that this is beta software, and not as stable as other alternatives. [MikTeX](https://miktex.org/){target="_blank" rel="noopener noreferrer"} is also a great distribution.

# Getting started

## Creating a document

To get started with a document, run

    doctor new <name>

to create a directory with the supplied name, containing the structure of a Doctor document workspace. Omitting `<name>` creates a document in the current directory.

This command will create the following structure inside the workspace:

- `secs/`: This directory will contain the sections of your document, each represented by a Markdown file.
- `assets/`: This is a directory for all the assets needed by your document. This could be everything from figures to CSL styles.
- `references.bib`: This is a BibTeX file containing the document's bibliography. See [Citation](#citation) for more info on how this is handled.
- `doctor.toml`: This configuration file specifies key aspects of your document, like the title, author(s), and more.

As a rule, you should not manually create, rename or move files in this directory - the exception being to place assets in the `assets` sub-directory. Rather, you should let Doctor handle the dirty work of maintaining the workspace, and focus on what you're best at: Writing amazing content!

## Adding and removing sections

A *section* in a Doctor document is just a single Markdown file in the `secs` directory. By design, this file should represent **one** single section of the document - this ensures a consistent structure that is simple to maintain. To add a section, run

    doctor add <name of section>

This will create a new Markdown file under `secs` and adds a top level header with the specified name. To include spaces in your header, simply wrap it in quotes, for example `doctor add "Results and discussion"`. A good rule of thumb is to only use subheaders below it, to stay in tune with having only *one* section per file. By default, every time the `add` command is run, the section is appended to the document. If you want to add a section to a specific position however, you need only supply the index of your desired position after the title, like this:

    doctor add <title of section> <index>

To get an overview of the document before adding a section at a specific position, see [Listing the document structure](#listing-the-document-structure).

When adding a secion named either "Abstract" or "Appendix", it will be prepopulated with some formatting that reflect these special section types. Try it out, I'm sure you'll be delighted!

## Removing sections

To remove a section, run the following

    doctor remove <identifier>

Here, `<identifier>` can be either the index of the section you want to remove or the name of the section. Once again, see [Listing the document structure](#listing-the-document-structure) for a description on how to get an overview of the project.
