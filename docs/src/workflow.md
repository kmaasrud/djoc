---
draft = true
---

MDoc documents are written in [Pandoc's flavor of Markdown](markdown/markdown.md), with a few syntactic delights sprinkled in. If at any time you feel this markup language is limiting, you can simply write inline LaTeX or HTML and it will be parsed as expected. However, I challenge you to avoid this as much as you can to get a feel for Markdown's refreshing simplicity, and to reassess how *fancy* your document really needs to be.

The MDoc tooling is focused on users familiar with the command line. As such, using a text editor with easy access to a terminal is highly recommended. Visual Studio Code has an option of opening a terminal "panel", for example, which is a great way of working with MDoc. Using the embedded terminal in an editor like Neovim also works smoothly.

A live-updating `watch` command will soon arrive to MDoc, but for the time being I personally just open the built PDF in my favorite PDF viewer and rebuild whenever I need to preview my changes. There are also a bunch of extensions for Visual Studio Code which give a live preview of your current section.
