---
title: Commands
toc: True
---

# `doctor new`

Used like:

```shell
doctor new <name>
```

This will create a new document inside a directory with the specified name. Omitting `<name>` creates a document in the current directory.

# `doctor build`

Used like:

```shell
doctor build
```

Will build the current document into a PDF.

# `doctor add`

Used like:

```shell
doctor add <name of section>
```

Adds a new section to your document, automatically assigning an index to it. If you want to specify which index you want it to have, run

```shell
doctor add <name of section> --at <index>
```

> You may also use `doctor add <name> -i <index>`, `doctor add <name> --at=<index>` or `doctor add <name> -i=<index>`.

This will add the new section at the specified index, reordering the other sections accordingly.

# `doctor remove`

Used like:

```shell
doctor remove <identifier> ...
```

Will search for a section that has an index or name that matches `<identifier>` and remove it, reordering the other sections accordingly. The `remove` command allows multiple arguments, meaning you can supply multiple identifiers, and Doctor will remove them one after one.

Removing a section requires confirmation, which Doctor will ask for. If you want to skip this step (e.g. when deleting a lot of sections), run with the `--confirm` or `-c` flag. This will proceed with removing sections without asking for confirmation.

# `doctor move`
