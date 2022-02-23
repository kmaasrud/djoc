# Changelog

## 0.2.0 (Unreleased)

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
