---
title: Configuring
toc: true
---

Your Doctor document can be configured quite extensively with the `doctor.toml` file. This file allows you to specify metadata, apply styling, supply information to Pandoc and/or the $\TeX$ engine, and much more. I've chosen the TOML specification since I consider it the most human-friendly configuration interface - it's legibility being the main draw.

The configuration file has six main *tables* (which is TOML terminology for a collection of key-value pairs following a header): `[meta]`, `[build]`, `[style]`, `[bib]`, `[latex]` and `[html]`. They contain the configurations as listed below.

# `[meta]`

These options relate to the metadata of your document.

| **Config name** | **Description** |
|--|:--|
| `title` | The document's title. |
| `author` | The document's author or a list of authors. |
| `date` | The date of your document. If the date is `"today"` or `"now"`, Doctor will insert the current date. |

Here's an example of a `[meta]` table:

```toml
[meta]
title = "Our amazing report"
author = ["Jane Doe", "John Doe"]
date = "February 17th 1998"
```

# `[build]`

These options allow you to tune how the document is built.

| **Config name** | **Description** |
|--|:--|
| `engine` | The $\TeX$ engine you want to use to build your PDF. The options are: `pdflatex`, `lualatex`, `xelatex`, `latexmk` and `tectonic`. If no engine is specified, Doctor will use `pdflatex` as the default. |
| `filename` | The filename you want for your exported document. You do not need to specify an extension, Doctor will automatically append `.pdf` when exporting as a PDF and `.html` when exporting as HTML. |
| `output-format` | The format of your exported document. The options are `"html"` or `"pdf"`. |
| `lua-filters` | Boolean that specifies whether or not to use the embedded Lua filters. This option is mainly for debugging. The default is `true`, and turning it `false` will stop some functionality like cross-referencing. |

Here's an example of a `[build]` table:

```toml
[build]
engine = "lualatex"
filename = "awesome-document"
output-format = "html"
lua-filters = false
```

# `[style]`

These are options that specify how you want your document presented.

| **Config name** | **Description** |
|--|:--|
| `two-column` | If this option is `true`, the PDF document will be formatted with two columns on each page. The default is `false`. |
| `number-sections` | Boolean that specifies whether or not you want the sections of your document numbered. If you want to use cross-referencing with sections, this option must be true. |
| `document-class` | The $\LaTeX$ document class you want to use. Can be any of the ones listed [here](https://ctan.org/topic/class). Beware that not all classes are tested with the Doctor syntax, so some might not work as expected. |
| `class-options` | Options for the document class. The available options depend on the chosen document class, but some commonly used ones are listed [here](https://en.wikibooks.org/wiki/LaTeX/Document_Structure#Document_Class_Options). |

Here's an example of a `[style]` table:

```toml
[style]
two-column = true
number-sections = true
document-class = "report"
class-options = "landscape"
```

# `[bib]`

These options all relate to citations and the bibliography of your document.

| **Config name** | **Description** |
|--|:--|
| `csl` | The CSL ([Citation Style Language](https://citationstyles.org/)) style to use for your citations. You can either use one of the CSL styles that come prepackaged with Doctor (listed [here](bib#csl)), a CSL file in your `assets` folder or a URL that points to a CSL file available on the internet. For local files, you need only specify the name, not the `.csl` extension. The default CSL style is the [Chicago Manual of Style 17th edition](https://csl.mendeley.com/styleInfo/?styleId=http%3A%2F%2Fwww.zotero.org%2Fstyles%2Fchicago-author-date). |
| `bibliography-title` | The title of the bibliography section. The default is no title. |
| `bibliography-file` | The name of your BibTeX file. All paths are relative to the `assets` directory. The default is `references.bib` |
| `include-bibliography` | Boolean that specifies whether or not you want your bibliography included in your document. The default is `true`. |

Here's an example of a `[bib]` table:

```toml
[bib]
csl = "apa"
bibliography-title = "References"
include-bibliography = false
```

# `[latex]`

These are options that are specific for $\LaTeX$ and PDF output.

| **Config name** | **Description** |
|--|:--|
| `packages` | A list of strings specifying the packages you want used. This is similar to `\usepackage{package}` in a $\LaTeX$ document, but much less verbose. If you want to supply options to you package inclusion, you can write them with brackets as you would normally. An example could be when you want to specify a language to Babel. In this case, you could write `packages = ["[norsk]{babel}"]`. |
| `header` | Whatever you want parsed by $\LaTeX$ as header content. For simple package inclusions, the above option is recommended, but if more advanced headers are required, you can use this option. To avoid having to escape macros or manually write newline characters, you can use a multiline literal string, which is surrounded by `'''`. |

Here's an example of a `[latex]` table:

```toml
[latex]
packages = ["graphicx", "placeins", "[utf8]{inputenc}"]
header = '''\makeatletter
\newcommand*{\centerfloat}{%
  \parindent \z@
  \leftskip \z@ \@plus 1fil \@minus \textwidth
  \rightskip\leftskip
  \parfillskip \z@skip}
\makeatother'''
```

# `[html]`

These are options that are specific for HTML output, but since you cannot output HTML yet, these do nothing at the moment.

| **Config name** | **Description** |
|--|:--|
| `header` | Whatever you want included in the `<head>` of you HTML document. To avoid having to escape macros or manually write newline characters, you can use a multiline literal string, which is surrounded by `'''`. |
