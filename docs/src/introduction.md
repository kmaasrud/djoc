---
title: Introduction and key concepts
---

Doctor aims at giving you the most streamlined writing experience possible. It is somewhat opinionated in how it enforces a specific workflow and workspace structure. This is simply so that you don't have to bother concerning yourself with structuring your project and keeping it organized manually - which I know from experience can be extremely time-draining.

You write in Pandoc Markdown, with a few Doctor specific syntax-delights sprinkled in. If at any time you feel this markup language is limiting, you can simply write some $\LaTeX$ or HTML and it will be parsed as expected. I do however challenge you to avoid this as much as you can to really feel how refreshingly simple Pandoc Markdown can be, and to reassess how *fancy* your document really needs to be. You can read more on my choice of Pandoc Markdown [here](markdown), and learn the markup rules in the sections under **Markdown**.

The Doctor tooling is focused on users familiar with the command line. As such, using a text editor with easy access to a terminal is highly adviced. Visual Studio Code has an option of opening a terminal "panel" for example, which is a great way of working with Doctor. Using the embedded terminal in an editor like Neovim also works very smoothly! For previewing your document, I personally just open the PDF in my favorite PDF viewer and rebuild whenever I need to see the changes. There are also a bunch of extensions for Visual Studio Code which give a live preview of your current section. Some more guides pertaining to different editors will arrive here shortly...

# Key concepts

## Documents {#documents}

In Doctor terminology, a *document* is simply a directory containing everything needed for Doctor to produce your desired output. The directory has the following structure:

- `secs/`: This directory will contain the sections of your document, each represented by a Markdown file.
- `assets/`: This is a directory for all the assets needed by your document. This could be figures, CSL styles, bibliographies or anything you need, really.
- `doctor.toml`: This configuration file specifies key aspects of your document, like the title, author(s), and more. For more info, see [Configuring](config)

As a rule, you should not manually create, rename or move files in a document - except within the `assets` directory of course. Rather, you should let Doctor handle the dirty work of maintaining the workspace, and focus on what you're best at: Writing amazing content!

## Sections {#sections}

A *section* in a Doctor document is just a single Markdown file in the `secs` directory. By design, this file should represent only **one** single section of the document. This ensures a consistent structure that is simple to maintain. A good rule of thumb is to start each section with the title of that section as a top level header, and only use subheaders below it.

Doctor automatically assigns an index to your new sections. The indices are used to determine the order in which the sections appear in the final output. If you insist, you can rename the files to reorder the sections yourself, but it is adviced to use the Doctor command line tool for this task. It will make sure the correct order is kept when adding, removing or moving sections.
