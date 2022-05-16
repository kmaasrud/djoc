MDoc uses Pandoc Markdown (with a few carefully selected additions) as it's markup language. This choice was a simple one; the creator of Markdown, [John Gruber](https://daringfireball.net/projects/markdown/syntax#philosophy), puts it best himself:

> Markdown is intended to be as easy-to-read and easy-to-write as is feasible. Readability, however, is emphasized above all else. A Markdown-formatted document should be publishable as-is, as plain text, without looking like it’s been marked up with tags or formatting instructions.

This is at the heart of most of my choices when creating MDoc, as I'm sure also was the case for [John MacFarlane](https://johnmacfarlane.net/) when he made the canonical additions that make up Pandoc Markdown.

But why yet another Markdown flavor? Well, the goals of Pandoc, and subsequently of MDoc, is to produce a more complex output than a simple HTML page. To conform with the abovementioned philosophy, writing inline HTML or TeX (although permitted) to achieve this complexity, is simply not good enough. Thus, a few additions are needed, but should come as second nature to those already familiar with Markdown.

This part of the documentation contains a modified copy of [Pandoc's User Guide](https://pandoc.org/MANUAL.html#pandocs-markdown). I've done this to explain the syntax at the perspective of MDoc's goals and to specify the changes and conventions MDoc introduces.
