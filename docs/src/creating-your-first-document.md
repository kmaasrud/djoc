To get started with Doctor, create a new document by running

```shell
doctor new "Awesome document"
```

This will create a new [document](introduction#documents) inside a directory conveniently named `Awesome document`. Next, add a section by running

```shell
doctor add introduction
```

This will create a new Markdown file named `00_introduction.md` under the directory `secs` and adds a top level header `# Introduction` to it. Go ahead and add some more Markdown content to this file. You can write whatever you want, so make it something fun and add some $\TeX$ math or code listings if you want! To get your document built into a PDF, simply run

```shell
doctor build
```

This will produce a PDF called `document.pdf` in your root directory. Open it with your preferred PDF viewer and marvel at how simple it was to create this beautiful $\LaTeX$-formatted PDF - no `\begin{document}` needed.

Now, you might not be quite as impressed as I make it out to be yet... For instance, the author is "AUTHOR", how do you change that? Have a look inside `doctor.toml`. This file is where you'll find your document's metadata, build and style instructions, bibliography details and many more options - having this in a separate file is a crucial step in ensuring separation of concerns. Change the `title`, `author` and `date` field into whatever you desire and run `doctor build` again to see the changes get applied.

You are now ready to add more sections... (WIP)
