+++
title = "Block quotes"

[menu.main]
name = "Block quotes"
weight = 29
parent = "markdown"
+++

Pandoc Markdown uses email conventions for block quotes. A block quotation is one or more paragraphs or other block elements (such as lists or headings), with each line preceded by a `>` character and an optional space.

```markdown
> This is a block quote. This
> paragraph has two lines.
>
> 1. This is a list inside a block quote.
> 2. Second item.
```

Block quotes can contain most other Markdown elements. Among them are other blockquotes, which means they can be nested, like this:

```markdown
> This is a block quote.
>
> > A block quote within a block quote.
```

Standard Markdown syntax does not require a blank line before a block quote. MDoc does require this (except, of course, at the beginning of the document). The reason for the requirement is that it is all too easy for a `>` to end up at the beginning of a line by accident (perhaps through line wrapping).
