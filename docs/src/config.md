In bigger document projects, you will have an `mdzk.toml` file at the root of your document. This file is meant to configure your document to look and behave the way you want. The available options are:

- [`title`](#title) (required)
- [`authors`](#authors)
- [`date`](#date)
- [`src`](#src)


### The `title` field (required) {#title}

This field specifies the title of your document. It will be used for the title at the top of your generated PDF and to derive the filename of your output (unless another filename is specified.) The title can contain [inline Markdown formatting](./markdown/formatting.md).

```toml
title = "My awesome document"
```

### The `authors` field {#authors}

This is list of strings containing the authors of this document. At the moment, each author is only described by their name, but in the future you will be able to specify more complex information about each author (see [#63](https://github.com/kmaasrud/mdoc/issues/63)).

```toml
authors = [ "John McClane", "Hans Gruber" ]
```

### The `date` field {#date}

Here you can set the date which will be showed in the title of the document. The date can be any string you want, but if it is formatted according to [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601), additional formatting is supported (see [`date-format`](#date-format)). If this field is set to the string `now`, the current date and time will be used.

```toml
date = "now"
```

### The `src` field {#src}

This is a special field that does not need to be explicitly set. It defines which files will get built into your document. `src` can either be one of the following:

- Nothing. This will deploy the default behavior where MDoc recursively searches for Markdown-files, starting from the document root. All files will get built into the finished document, ordered alphabetically by their path.
- A path to a directory, relative to the `mdzk.toml` file. This will tell MDoc to search for Markdown-files recursively inside only this directory. This is useful if you want to include a README or any other Markdown-file inside the document, but avoid building it as part of the document.

    ```toml
    src = "path/to/dir"
    ```

- A list of paths. Each path is relative to the `mdzk.toml` file and must point to other valid files. Using this list, you can explicitly define which sections you want included in your document. The order of the list is preserved, which makes this an excellent method to provide a custom (non-alphabetical) order your sections.

    ```toml
    src = [
      "path/to/first-section.md",
      "second-section.md,
    ]
    ```
