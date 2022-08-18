It is time for academic writing to step out of it's archaic ways and into the modern world! With the rise of Markdown as the de facto markup language of the internet, why should academia lag behind. You should be able to write scientific content, confident that it is publishable both physically and on the web. Additionally, you should have the comfort of knowing your content is readable in it's raw form; this will help it stand the test of time.

### The issue with $\LaTeX$

[LaTeX](https://www.latex-project.org/) is incredible, but it has three key flaws:

- It's tooling is bloated and fractioned (to say the least);
- Handling bigger projects is a hastle;
- The syntax is obscure.

The latter issue is exemplified on it's own introduction page[^latex]. Consider this example:

```
\documentclass{article}
\begin{document}
   Hello world!
\end{document}
```

Four lines for a simple hello world? The situation gets much hairier when we start to introduce macros to the preamble, and insert environments to get different content displayed. Compare the above with the same document written in Markdown:

```markdown
Hello world!
```

Much better!

### The solution

What if we used TeX/LaTeX only as an intermediate step when producing a document? It would then function like an *assembly language* of sorts, and we could opt for a *higher level* markup language that is simpler to both read and write. With this, we gain simplicity, without abandoning the centuries of typesetting expertise that is TeX/LaTeX.

This is the solution MDoc proposes as a way into the modern world. Through the help of the incredible document converter [Pandoc](https://pandoc.org/) and the amazing TeX/LaTeX engine made by the [Tectonic](https://github.com/tectonic-typesetting/tectonic) team, we have a solid foundation for acheiving this goal. With MDoc, we also provide tooling for handling all the nitty gritty of maintaining bigger documents, like bibliography handling, metadata and styling.

[^latex]: You can find the page [here](https://www.latex-project.org/about/). I've modified the example a bit, for brevity.
