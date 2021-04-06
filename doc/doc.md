---
title: Doctor documentation
---

This wiki contains guides on getting started, documentation on using KODB and also acts as a general wiki for writing texts using this tool. Beware that this wiki is very much under construction, but hopefully the pages here prove useful.

# Installation

KODB depends on [Pandoc](https://pandoc.org/) for parsing the Markdown documents and [Tectonic](https://tectonic-typesetting.github.io/) for producing the PDF output. Thus, they need to be installed and available in the PATH for KODB to work.

- Pandoc has lots of installation options for almost every platform. See [Installing pandoc](https://pandoc.org/installing.html) and follow the instructions for your platform.
- Tectonic is quite a young software and not as easy to install, but it should work for Windows, Mac and most Linux distributions. See [Installing Tectonic](https://tectonic-typesetting.github.io/en-US/install.html) and find the installation method best suited for you. (Quick-tip: On Windows, an older version of Tectonic is available on [Chocolatey](https://chocolatey.org/packages/tectonic))

When this is done make sure you have a working installation of Python (version 3 is required) and its package manager, PIP. Now you can do the final step and install KODB:

    pip install kodb

To verify that everything works correctly, run `kodb --check-dependencies`.

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
