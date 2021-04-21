---
title: Code
toc: true
---

# Code blocks

## Indented code blocks

A block of text indented four spaces (or one tab) is treated as a code block. For example,

```markdown
Paragraph here

    if (a > 3) {
      moveShip(5 * gravity, DOWN);
    }
```

The initial (four space or one tab) indentation is not considered part of the verbatim text, and is removed in the output.

## Fenced code blocks

In addition to standard indented code blocks, Doctor supports fenced code blocks. These begin with a row of three or more backticks (<code>```</code>) and end with a row of backticks that must be at least as long as the starting row. Everything between these lines is treated as code and no indentation is necessary:

```````markdown
```
if (a > 3) {
  moveShip(5 * gravity, DOWN);
}
```
```````

Like regular code blocks, fenced code blocks must be separated from surrounding text by blank lines. If the code itself contains a row of backticks, just use a longer row of tildes or backticks at the start and end:

`````````markdown
``````
```
if (a > 3) {
  moveShip(5 * gravity, DOWN);
}
```
``````
`````````

If you want to specify the language of the code block, you can write

``````rust
```rust
match foo {
    Some(a) => println!("Something: {}", a),
    None => println!("Nothing")
}
```
``````

This is just a shorthand for attaching the language as a class, like `{.rust}`. You can attach further attributes to a fenced code block by using this syntax:

```````markdown
```{#mycode .haskell .numberLines startFrom="100"}
qsort []     = []
qsort (x:xs) = qsort (filter (< x) xs) ++ [x] ++
               qsort (filter (>= x) xs)
```
```````

Here `mycode` is an identifier, `haskell` and `numberLines` are classes, and `startFrom` is an attribute with value `100`.

# Inline code

To write inline code, put the text between backticks, e.g.

```markdown
What is the difference between `>>=` and `>>`?
```

If you want to include a literal backtick as part of the code, use double backticks (the spaces before and after the enclosed backtick will be ignored.)

```markdown
Here is a literal backtick `` ` ``.
```

> **NOTE**: Backslash-escapes (and other Markdown constructs) do not work inside verbatim (code) contexts:
>
> ```markdown
> This is a backslash followed by an asterisk: `\*`.
> ```

As with [code blocks](#code-blocks), you can attach attributes to inline code as well. The syntax is similar:

```markdown
This is Haskell code: `<$>`{.haskell}
```
