# Changelog

## 0.1.2 (Unreleased)

### Added

- New configuration field `date-format` under the `style` table. This option is used to specify how you want your date formatted. The format strings follows [this spec](https://docs.rs/chrono/latest/chrono/format/strftime/index.html). The default formatting is `"%e %B %Y"`.

### Changed

- [#59](https://github.com/kmaasrud/mdoc/issues/59) MDoc now has a custom interface with Pandoc, and Pandoc now handles most of the Markdown to LaTeX conversion. This should lead to less unexpected behavior, and make it easier for me to change to another LaTeX generation method, if I want to.
- When `date` is a RFC 3339 compliant string, it will get parsed into a datetime and formatted according to the `date-format`. Timezones are currently ignored.

### Improved

- Warnings from Pandoc are now caught by MDoc and printed as expected.

## 0.1.1 (2020-12-17)

### Changed

- Default `mdoc.toml` file is now much simpler and includes a helpful comment.
- Chapters are now sorted by path, which makes handling projects actually possible without specifying each file manually.

## 0.1.0

The initial release of MDoc.
