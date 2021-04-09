---
title: Doctor documentation
---

Welcome to the Doctor documentation! Here you will find everything you need to know about writing documents with Doctor.

# Installation

To download Doctor, either take a look at one of the pre-compiled binaries on [GitHub](/), or download it with the [Go tool](https://golang.org/doc/install) by running

	go install github.com/kmaasrud/doctor

Doctor needs one crucial component in order to work correctly, namely [Pandoc](https://pandoc.org/). Pandoc is actually so important that Doctor automatically downloads it for you if it is not already present in your system. Don't worry, if you have it installed already, you will not get a double.

Additionally, if you want to produce PDFs, you will need a $\LaTeX$ engine. The fastest alternative is [TeXLive's](https://tug.org/texlive/) `pdflatex`, which you probably already have installed. For a lighter alternative, which is almost as fast and automatically installs missing packages for you, have a look at downloading [Tectonic](https://tectonic-typesetting.github.io/)

When this is done make sure you have a working installation of Python (version 3 is required) and its package manager, PIP. Now you can do the final step and install KODB:

    pip install kodb

To verify that everything works correctly, run `kodb --check-dependencies`.

# About Pandoc

Pandoc is perhaps the best document converter one can find, and also sports one of the most thorough and well thought out Markdown specifications. 

# Creating a document

To get started with a document, run

    kodb new <name>

to create a directory with the supplied name, containing the structure of a KODB document workspace. You can also run

    kodb init

to use the current working directory as a root of the document workspace. This command will create the following structure inside the workspace:

- `src/`: This directory will contain the Markdown-files that make up your project.
- `assets/`: This is a directory for all the assets needed by your document. This could be everything from figures to CSL styles.
- `kodb.yaml`: This is a YAML file that holds all your document's configuration. It serves the purpose as Pandoc's YAML file, but with some extensions specific to KODB.

KODB also makes a `references.bib` file in the `assets` directory for you to place your references in. See [Citation](https://github.com/kmaasrud/kodb/wiki/Citation) for more details on this.

As a rule, you should not manually create, rename or move files in the KODB document workspace - with the exception of adding files to the `assets` directory. Rather, you should let KODB handle the dirty work of maintaining the workspace, and focus on what you're best at: Writing amazing content! However, you might notice that the document is a bit empty for the time being. To solve this, see [Adding and removing sections](https://github.com/kmaasrud/kodb/wiki/Adding-and-removing-sections).

# Adding and removing sections

A *section* in a KODB document is just a single Markdown file in the `src` directory, but that file should also represent only **one** single section of the document. A good rule of thumb is to start each section with a top level header (a single `#`), and only use subheaders below it. This ensures a consistent file structure that accurately depicts your document.

## Adding a section

To create a section, use the `add` command:

    kodb add <title of section>

This will add a Markdown file to your `src` directory with a header and filename corresponding to the supplied section title. To include spaces in your title, simply wrap it in quotes, for example `kodb add "Results and discussion"`. By default, every time the `add` command is run, the section is appended to the document. If you want to add a section to a specific position however, you need only supply the index of your desired position after the title, like this:

    kodb add <title of section> <index>

To get an overview of the document before adding a section at a specific position, see [Listing the document structure](https://github.com/kmaasrud/kodb/wiki/Listing-the-document-structure).

### Special sections

When adding a secion named either "Abstract" or "Appendix", it will be prepopulated with some formatting that reflect these special section types. Try it out, I'm sure you'll be delighted!

## Removing sections

To remove a section, run the following

    kodb remove <identifier>

Here, `<identifier>` can be either the index of the section you want to remove or the name of the section. See [Listing the document structure](https://github.com/kmaasrud/kodb/wiki/Listing-the-document-structure) for a description on how to get an overview of the project.
