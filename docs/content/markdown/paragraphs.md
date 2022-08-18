+++
title = "Paragraphs"

[menu.main]
parent = "markdown"
weight = 22
+++

A paragraph is one or more lines of text followed by one or more blank lines. Newlines are treated as spaces, so you can reflow your paragraphs as you like. Pandoc's Markdown is strict, so if you need a hard line break, you have three options:

- Use two newlines or more (essentially separating each paragraph by a blank line). This is the recommended and most readable way.

    ```Markdown
    This is the first paragraph

    This is the second paragraph
    ```

- Use a backslash directly followed by a single newline. **Note**: in multiline and grid table cells, this is the only way to create a hard line break, since trailing spaces in the cells are ignored.

    ```Markdown
    This is the first paragraph\
    This is the second paragraph
    ```

- Use two or more spaces at the end of a line. This method should only be used if absolutely necessary, as it makes the document source less readable.
