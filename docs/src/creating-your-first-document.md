---
title: Creating your first document
toc: true
toc-depth: 2
---

# Getting started

## Creating a document {#doctor-new}

In Doctor terminology, a *document* is simply a directory containing everything needed for Doctor to produce your desired output. To get started, run

    doctor new <name>

This will create a new document inside a directory with the specified name. Omitting `<name>` creates a document in the current directory.

The `new` command will create the following file structure inside your document:

- `secs/`: This directory will contain the sections of your document, each represented by a Markdown file.
- `assets/`: This is a directory for all the assets needed by your document. This could be everything from figures to CSL styles.
- `references.bib`: This is a BibTeX file containing the document's bibliography. See [Citation](#citation) for more info on how this is handled.
- `doctor.toml`: This configuration file specifies key aspects of your document, like the title, author(s), and more. For more info, see [Configuring with the TOML file](#config)

<!-- TODO: Line-block here about default doc structure, when that is implemented -->

As a rule, you should not manually create, rename or move files in a document - the exception being to place assets in the `assets` directory. Rather, you should let Doctor handle the dirty work of maintaining the workspace, and focus on what you're best at: Writing amazing content!

## Adding sections {#doctor-add}

A *section* in a Doctor document is just a single Markdown file in the `secs` directory. By design, this file should represent only **one** single section of the document. This ensures a consistent structure that is simple to maintain. A good rule of thumb is to start each section with the title of that section as a top level header, and only use subheaders below it.

To add a section, run

    doctor add <name of section>

This will create a new Markdown file under `secs` and adds a top level header with the specified name. To include spaces in your section name, simply wrap it in quotes, for example `doctor add "Results and discussion"`.

Doctor automatically assigns an index to your new sections. The indices are used to determine the order in which the sections appear in the final output. If you insist, you can rename the files to reorder the sections yourself, but it is adviced to use the Doctor command line tool for this task. It will make sure the correct order is kept when adding, removing or moving sections.

By default, every time the `add` command is run, the section is appended to the document. If you want to add a section to a specific position however, you need only use the `--at` flag to supply the index of your desired position after the title, like this:

    doctor add <title of section> --at <index>

| You may also use `doctor add <name> -i <index>`, `doctor add <name> --at=<index>` or `doctor add <name> -i=<index>`.

## Removing sections {#doctor-remove}

To remove a section, run the following

    doctor remove <identifier>

`<identifier>` can be either the index of the section you want to remove or the name of the section. Doctor will ask you for confirmation each time you remove a section. If you want to skip this, you can add the `--confirm` or `-c` flag to your command.

## Building the document {#doctor-build}

To get your document built into a PDF, simply run

	doctor build

This will produce `main.pdf` in your root directory.

## Configuring with the TOML file {#config}

To configure your document, use the `doctor.toml` file at the document root. It uses the [TOML](https://toml.io/en/) specification, which has a friendly and readable syntax. At the moment, Doctor only has a limited selection of configuration options. You do not need to specify any of them, but keep the `doctor.toml` file around - Doctor will not function if you delete it.

#### `[document]`

These are options affecting the content and look of your document. The following fields are supported:

- `title`: This is a string representing the title of your document.

    > *Example*: `title = "My amazing report!"`

- `author`: This can either be a string specifying the document's author, or a list of strings specifying several authors.

    > *Example*: `author = ["Jane Doe", "John Doe"]`

- `date`: This is the date listed on your document, represented by a string. If the date is `"today"`, Doctor will insert the current date.

    > *Example*: `date = "February 17th 1998"`

#### `[build]`

These are options affecting the build process. The following fields are supported:

- `engine`: A string specifying which PDF engine you would like to use. The options are:

    - `pdflatex`
    - `lualatex`
    - `xelatex`
    - `latexmk`
    - `tectonic`

    If no engine is specified, the default engine is `tectonic`.

    > *Example*: `engine = "lualatex"`
