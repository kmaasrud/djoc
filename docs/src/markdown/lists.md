---
title: Lists
toc: true
---

# Ordered lists

To make an ordered list of items, write a decimal number, followed by a period, a space and the content you want as part of that list item.

```markdown
1.  one
2.  two
3.  three
```

This will produce a “compact” list. If you want a “loose” list, in which each item is formatted as a paragraph, put spaces between the items:

```markdown
5.  one

34.  two

8.  three
```

Only the first number is parsed, which means the above list will be formatted as the sequence `5.`, `6.` and `7.`. You can make use of this by specifying only the starting number, and using a `#.` in place of the following numbers. However, in the name of legibilty I recommend writing the numbers as you intend them.

Multiple styles of alphanumeric sequences are allowed. These are uppercase/lowercase letters and roman numerals, in addition to normal Arabic numerals. You can use the regular dot notation (`2.`), enclose the index in parentheses (`(i)`) or follow it by a single closing-parentheses (`a)`). This variation of styles is utilized when one needs a new list directly following another or to make a sublist. The following are all valid lists:

```shell
 9) Ninth
10) Tenth
11) Eleventh
       i. subone
      ii. subtwo
     iii. subthree

(2) Two
(5) Three
1.  One
#.  Two
```

which gives the following output:

---

 9) Ninth
10) Tenth
11) Eleventh
       i. subone
      ii. subtwo
     iii. subthree

(2) Two
(5) Three
1.  One
#.  Two

---

# Unordered lists

An unordered list is a list of bulleted list items that are not numbered. They work exactly like ordered lists (except the numbering of course,) but use `*`, `+` or `-` as markers.

```markdown
- one
- two
- three
```

The bullets need not be flush with the left margin; they may be indented one, two, or three spaces. The bullet must be followed by whitespace.

## Task lists

You can also make unordered lists that have a checkmark, in order to create a task list. The Github-Flavored Markdown syntax is used, which is as follows

```markdown
- [ ] an unchecked task list item
- [x] checked item
```

# Block content in lists

A list item may contain multiple paragraphs and other block-level content. However, subsequent paragraphs must be preceded by a blank line and indented to line up with the first non-space content after the list marker.

```markdown
* First paragraph.

  Continued.

* Second paragraph. With a code block, which must be indented
  eight spaces:

      { code }
```

List items may include other lists, like in the last ordered list example. In this case the preceding blank line is optional. The nested list must be indented to line up with the first non-space character after the list marker of the containing list item.

```markdown
* fruits
  + apples
    - macintosh
    - red delicious
  + pears
  + peaches
* vegetables
  + broccoli
  + chard
```

Markdown allows you to write list items “lazily,” and reflow your text with a single newline instead of indenting continuation lines. However, if there are multiple paragraphs or other blocks in a list item, the first line of each must be indented.

```shell
+ A lazy, lazy, list
item.

+ Another one; this looks
bad but is legal.

    Second paragraph of second
list item.
```

But what if you want to put an indented code block directly after a list? Consider the following:

```markdown
-   item one
-   item two

    { my code block }
```

`{ my code block }` will here get treated as the second paragraph of the second list item. In order to “cut off” the list after item two, you can insert some non-indented content (an HTML comment would be a great choice,) which won’t produce visible output in any format. An example of this could be:

```markdown
-   item one
-   item two

<!-- end of list -->

    { my code block }
```

# Extra list types

## Definition lists

Definition lists are made of terms and definitions of these terms, much like in a dictionary. A simple definition list is made of a single-line term followed by a colon and the definition for that term.

```markdown
Apple

:   Pomaceous fruit of plants of the genus Malus in the family Rosaceae.

Orange

:   The fruit of an evergreen tree of the genus Citrus.

        { some code, part of Definition 2 }

    Third paragraph of definition 2.
```

Each term must fit on one line, which may optionally be followed by a blank line, and must be followed by one or more definitions. A definition begins with a `:` or `~`, which may be indented one or two spaces. A term may have multiple definitions, and each definition may consist of one or more block elements (paragraph, code block, list, etc.), each indented four spaces or one tab stop. The body of the definition (including the first line, aside from the `:` or `~`) should be indented four spaces. However, as with other Markdown lists, you can “lazily” omit indentation except at the beginning of a paragraph or other block element:

```markdown
Term 1

:   Definition
with lazy continuation.

    Second paragraph of the definition.
```

If you leave space before the definition (as in the example above), the text of the definition will be treated as a paragraph. In some output formats, this will mean greater spacing between term/definition pairs. For a more compact definition list, omit the space before the definition:

```markdown
Term 1
  ~ Definition 1

Term 2
  ~ Definition 2a
  ~ Definition 2b
```

Note that space between items in a definition list is required.

## Numbered example lists

The special list marker `@` can be used for sequentially numbered examples. The first list item with a `@` marker will be numbered 1, the next 2, and so on, throughout the document. The numbered examples need not occur in a single list; each new list using `@` will take up where the last stopped. So, for example:

```markdown
(@)  My first example will be numbered (1).
(@)  My second example will be numbered (2).

Explanation of examples.

(@)  My third example will be numbered (3).
```

Numbered examples can be labeled and referred to elsewhere in the document:

```markdown
(@good)  This is a good example.

As (@good) illustrates, ...
```

The label can be any string of alphanumeric characters, underscores, or hyphens.

> **Note**: continuation paragraphs in example lists must always be indented four spaces, regardless of the length of the list marker. This is because example labels tend to be long, and indenting content to the first non-space character after the label would be awkward.
