---
title: Formatting
toc: true
---

# Bold and italic text

To make text *italic*, surround it with single `*` or `_` symbols, like this:

```markdown
This text is _italicized with underscores_, and this
is *italicized with asterisks*.
```

Surrounding with double `*` or `_` symbols, produces **bold** text, e.g.

```markdown
This is **bold text** and __with underscores__.
```

A `*` or `_` character surrounded by spaces, or backslash-escaped, will not trigger emphasis:

```shell
This is * not emphasized *, and \*neither is this\*.
```

Because `_` is sometimes used inside words or identifiers, Pandoc will not interpret `_` as a marker if it's surrounded by characters. For this reason, it is advised to use `*` to avoid having to shift markers throughout the text.

# Strikeout

You can ~~strikeout~~ text by surrounding it with `~~`. For example:

```markdown
This ~~is deleted text.~~
```

# Superscript and subscript

^Super^scripts may be written by surrounding the superscripted text by `^` characters (intuitively enough); ~sub~scripts may be written by surrounding the subscripted text with `~` characters (not as intuitive.) Thus, for example,

```markdown
H~2~O is a liquid.  2^10^ is 1024.
```

> **Note**: The text between `^...^` or `~...~` may not contain spaces or newlines. If the superscripted or subscripted text contains spaces, these spaces must be escaped with backslashes. (This is to prevent accidental superscripting and subscripting through the ordinary use of `~` and `^`, as well as bad interactions with footnotes.) Thus, if you want the letter P with ‘a cat’ in subscripts, you write `P~a\ cat~`, not `P~a cat~`.
