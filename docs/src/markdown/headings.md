---
title: Headings
toc: true
---

There are two kinds of headings: [Setext](#setext-headers) and [ATX](#atx-headers). Both heading types can contain inline formatting, such as emphasis (see [Inline formatting](inline-formatting)).

> **Note**: You of course choose to use whatever style you prefer, but as a convention, you should use [ATX headers](#atx-headers) when writing with Doctor.

# ATX headers

An ATX-style heading consists of one to six `#` signs and a line of text, optionally followed by any number of `#` signs. The number of `#` signs at the beginning of the line is the heading level:

```markdown
## A level-two heading

### A level-three heading ###
```

The heading text can contain formatting:

```markdown
# A level-one heading with a [link](/url) and *emphasis*
```

Standard Markdown syntax does not require a blank line before a heading. Doctor does require this (except, of course, at the beginning of the document). The reason for the requirement is that it is all too easy for a `#` to end up at the beginning of a line by accident (perhaps through line wrapping). Consider, for example:

```
I like several of their flavors of ice cream:
#22, for example, and #5.
```

You should also include a blank line *after* the heading. In addition, many Markdown implementations do not require a space between the opening `#` signs of an ATX heading and the heading text, so that `#5` bolt and `#hashtag` count as headings. Doctor and Pandoc do require the space.

# Setext headers

A setext-style heading is a line of text “underlined” with a row of = signs (for a level-one heading) or - signs (for a level-two heading):

```markdown
A level-one heading
===================

A level-two heading
-------------------
```

When writing a Doctor document, you should preferably not use these types of headers.

# Identifiers and cross-referencing

## Identifiers

Headings can be assigned attributes using the following syntax at the end of the line containing the heading text:

    {#identifier .class .class key=value key=value}

Thus, for example, the following headings will all be assigned the identifier `foo`:

```markdown
# My heading {#foo}

## My heading ##    {#foo}

My other heading   {#foo}
---------------
```

> **Note**: All headers are automatically assigned an identifier corresponding to their slug cased title (e.g. `# This is my header!` will have the identifier `#this-is-my-header`).

## Linking and cross-referencing

These identifiers can be used to link to a header, e.g.

```markdown
# Foo section {#foo}

[This](#foo) is a link to the above section.
```

When [section numbering](config#number-sections) is enabled, you can also easily cross-reference a section by writing `[@sec:foo]` (the `sec:` part is required) to get the number of the section automatically inserted.

## Avoid numbering of headers

Headings with the class `unnumbered` will not be numbered, even if [`number-sections`](config#number-sections) is specified in `doctor.toml`. A single hyphen (`{-}`) is equivalent to this, so the following headers,

```markdown
# My heading {-}

# My heading {.unnumbered}
```

will never be numbered, regardless of the options supplied in [`doctor.toml`](config).
