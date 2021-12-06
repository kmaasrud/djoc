There are two kinds of headings: [Setext](#setext-headers) and [ATX](#atx-headers). They are equivalent, so choose the style you prefer. Both heading types can contain inline [[Formatting|formatting]], such as *emphasis*, or [[Math|math]].

## ATX headers

The most commonly used header style, an ATX heading consists of one to six `#` signs and a line of text, optionally followed by any number of `#` signs. The number of `#` signs at the beginning of the line is the heading level:

```markdown
## A level-two heading

### A level-three heading ###
```

> 📄 MDoc requires a blank line before and after a heading (except, of course, at the beginning of the document.) Additionally, a space between the opening `#` signs of an ATX heading and the heading text is required.

## Setext headers

A setext-style heading is a line of text “underlined” with a row of = signs (for a level-one heading) or - signs (for a level-two heading):

```markdown
A level-one heading
===================

A level-two heading
-------------------
```

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

> 📄 All headers are automatically assigned an identifier corresponding to their slug cased title (e.g. `# This is my header!` will have the identifier `#this-is-my-header`.)

## Numbering of headers

You can enable numbering of headers by setting `number-sections = true` under `[style]` in `mdoc.toml`. If you want to avoid numbering on certain headers, simply specify the class `unnumbered` on the,. A single hyphen (`{-}`) is equivalent to this, so the following headers:

```markdown
# My heading {-}

# My heading {.unnumbered}
```

will never be numbered.
