To get started with a new document, run the following command

```shell
mdoc init "Awesome document"
```

This will create directory named `Awesome document`, containing a file called `mdoc.toml` and a directory called `src`.

> 📖 If you want to initialize a document in the current working drive, simply run `mdzk init` without any arguments.

Next, open some Markdown files in your favorite editor. These can be located anywhere inside the document directory, but the `src` folder is the conventional place to put them. Go ahead and write content! You can write whatever you want, so make it something fun and be sure to add some $\TeX$ math or code listings if you want! To get your document built into a PDF, simply run

```shell
mdoc build
```

This will produce a PDF called `awesome-document.pdf` in your root directory. Open it with your preferred PDF viewer and marvel at how simple it was to create this beautiful PDF - no `\begin{document}` needed!

Now, you might not be blown away yet... How do I configure this document to look the way I like? Have a look inside `mdoc.toml`. This file is where you'll find your document's metadata, build instructions, styling, bibliography details and many more options. Try changing the `title` field into something else, and run `mdoc build` again to see the changes get applied.
