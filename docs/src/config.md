In bigger document projects, you will have an `mdoc.toml` file at the root of your document. This file is meant to configure your document to look and behave the way you want. The available options are:

- [`title`](#title) (required)
- [`authors`](#authors)
- [`date`](#date)
- [`src`](#src)
- [`[build]`](#build)
    - [`filename`](#build-filename)
- [`[bib]`](#bib)
    - [`csl`](#bib-csl)
    - [`src`](#bib-src)
- [`[style]`](#style)
    - [`number-sections`](#style-number-sections)
    - [`date-format`](#style-date-format)
    - [`document-classe`](#style-document-class)
- [`[latex]`](#latex)
    - [`packages`](#latex-packages)
    - [`head`](#latex-head)


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
- A path to a directory, relative to the `mdoc.toml` file. This will tell MDoc to search for Markdown-files recursively inside only this directory. This is useful if you want to include a README or any other Markdown-file inside the document, but avoid building it as part of the document.

    ```toml
    src = "path/to/dir"
    ```

- A list of paths. Each path is relative to the `mdoc.toml` file and must point to other valid files. Using this list, you can explicitly define which sections you want included in your document. The order of the list is preserved, which makes this an excellent method to provide a custom (non-alphabetical) order your sections.

    ```toml
    src = [
      "path/to/first-section.md",
      "second-section.md,
    ]
    ```
### The `[build]` table {#build}

This table contains configuration related to the build process of your document.

#### The `filename` field {#build-filename}

With this field you can set the filename of your built output. You do not need to specify an extension, MDoc will handle that for you based on the output type. By default, MDoc will use the [kebab cased](https://en.wikipedia.org/wiki/Letter_case#Kebab_case) [`title`](#title) field as the filename

```toml
[build]
filename = "my-document"
```

Running `mdoc build` with the TOML above would build a file called `my-document.pdf`.

### The `[bib]` table {#bib}

Under this table, you can set configuration values for MDoc's bibliography handling.

#### The `csl` field {#bib-csl}

If this field points to a valid [CSL]() source, MDoc will use this a the style for your citations. You can set the field to any of these:

- Any identifier listed in [citation-style-language/styles](https://github.com/citation-style-language/styles). MDoc will fetch the source and load the CSL style for you. You do not need to specify the `.csl` extension. The example below sets your style to [American Psychological Association 7th edition](https://github.com/citation-style-language/styles/blob/master/apa.csl).

    ```toml
    [bib]
    csl = "apa"
    ```

- A valid URL pointing to a CSL file.

    ```toml
    [bib]
    csl = "https://domain.com/this/points/to/apa.csl"
    ```

- The path to a CSL in your filesystem. The path is relative to the `mdoc.toml` file.

    ```toml
    [bib]
    csl = "./some/dir/apa.csl"
    ```

#### The `src` field {#src}

**NOT YET IMPLEMENTED**. MDoc does not read this field yet (see [#54](https://github.com/kmaasrud/mdoc/issues/54)): Here you can set the source of your bibliography. The source can be any of the following:

- Nothing. MDoc will search recursively for `.bib` and `.bibtex` files from the root directory. Any bibliography file found will be read and added to the document.
- A path to a directory, relative to the `mdoc.toml` file. MDoc will search recursively inside this directory and deploy the method explained above.

    ```toml
    [bib]
    src = "path/to/bib/dir"
    ```

- A list of paths. Each path is relative to the `mdoc.toml` file and must point to valid bibliography files. Each file liste will get read and added to the document.

    ```toml
    [bib]
    src = [
        "path/to/first.bib"
        "path/second.bibtex"
    ]
    ```

### The `[style]` table {#style}

This table contains all configuration fields related to styling your document.

#### The `number-sections` field {#style-number-sections}

A boolean, this field determines whether sections are numbered or not. The default is `false`.

```toml
[style]
number-sections = true
```
