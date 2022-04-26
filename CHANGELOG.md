# Changelog

## 0.3.0 (2022-04-26)

### **Notable changes**

- The `--tex` flag is no longer supported by the CLI. Use `--output tex` instead.

### Features

- Added `output` field to the `[build]` table in `mdoc.toml`. This field currently supports the output formats `html`, `tex` (or `latex`) and `pdf`. The default is `pdf`.

- Added `-o`/`--output` flag. It overrides the `output` field described above (or sets it to something other than `pdf` in the case of single-file builds.)

    `46b375c`

- [#67](https://github.com/kmaasrud/mdoc/issues/67): By default, a title is inserted into TeX/PDF output by using the `\maketitle` macro. If you want to produce a title with a custom macro or LaTeX script, you can set the `title-script` field under the `[latex]`. This string will replace `\maketitle`.

    7266c52

- [#57](https://github.com/kmaasrud/mdoc/issues/57): The new version cleans up more of the messy LaTeX messages by default. If you want to see everything from the TeX engine printed, you can toggle tidying by setting the `tidy-logs` field under the `[build]` table to `false`.

    `a91568e` `5e098a6`

- [#64](https://github.com/kmaasrud/mdoc/issues/64): Setting the `csl` field to a local file (path is relative to `mdoc.toml`) now loads that CSL file and uses it as the style definition.

    `faf487f`

- Preliminary support for HTML output. You can try it out with `mdoc build -o html`, but note that this is still very WIP.

    `c6c78e3` `7f7899c` `22a359c` and more

- Logging now respects the enviroment variable [`NO_COLOR`](https://no-color.org/).

    `93c0af8`

### Fixes

- Erroneous unwrap in `get_csl` is now handled properly. The function assumed it was being run inside a document project (directory with `mdoc.toml` in it), which made MDoc panic when building single files and trying to find a CSL file.

    `8ea1277`

- A few tweaks to the code internals and the `release` profile should hopefully increase performance slightly.

## 0.2.0 (2022-03-11)

### Features

- [#61](https://github.com/kmaasrud/mdoc/issues/61): Added `document-class` option under `[style]` in `mdoc.toml`. This allows setting the LaTeX document class. You can find a comprehensive list of different classes [here](https://ctan.org/topic/class).

    4d83eba: `feat: add document-class option in config`

### Fixes

- [#62](https://github.com/kmaasrud/mdoc/issues/62): Formatting in the title or author field is no longer escaped.

    9362675: `Fix lack of formatting in metadata fields`

## 0.1.2 (2022-01-19)

### Added

- New configuration field `date-format` under the `style` table. This option is used to specify how you want your date formatted. The format strings follows [this spec](https://docs.rs/chrono/latest/chrono/format/strftime/index.html). The default formatting is `"%e %B %Y"`.

### Changed

- [#59](https://github.com/kmaasrud/mdoc/issues/59) MDoc now has a custom interface with Pandoc, and Pandoc now handles most of the Markdown to LaTeX conversion. This should lead to less unexpected behavior, and make it easier for me to change to another LaTeX generation method, if I want to.
- When `date` is a RFC 3339 compliant string, it will get parsed into a datetime and formatted according to the `date-format`. Timezones are currently ignored.

### Improved

- Warnings from Pandoc are now caught by MDoc and printed as expected.

### Fixed

- [#58](https://github.com/kmaasrud/mdoc/issues/58): Some scaling artifacts would sometimes appear on images. This is now fixed by virtue of Pandoc fully handling the LaTeX conversion.

## 0.1.1 (2021-12-17)

### Changed

- Default `mdoc.toml` file is now much simpler and includes a helpful comment.
- Chapters are now sorted by path, which makes handling projects actually possible without specifying each file manually.

## 0.1.0

The initial release of MDoc.
