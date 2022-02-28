It is time for academic writing to step out of it's archaic ways and into the modern world! With the rise of Markdown as the de facto markup language of the internet, why should academia lag behind. You should be able to write scientific content, confident that it is publishable both physically and on the web, along with the comfort of knowing the content is readable in it's raw form and will stand the test of time.

### The issue with $\LaTeX$

[$\LaTeX$](https://www.latex-project.org/) is incredible, but it has three key flaws.

- It's tooling is fractioned (to say the least).
- It is difficult to learn.
- The syntax is obscure.

The latter issue is exemplified on [it's own introduction page](https://www.latex-project.org/about/):

```tex
\documentclass{article}
\title{Cartesian closed categories and the price of eggs}
\author{Jane Doe}
\date{September 1994}
\begin{document}
   \maketitle
   Hello world!
\end{document}
```

All this for a simple hello world... I am intentionally ignoring the function of all these lines; the content should be front and center - not obscured by a bunch of boilerplate and complex macros. Compare the above with the same document written in Markdown[^1]:

```markdown
Hello world!
```

How refreshing!

[^1]: Without the title, of course, but we'll get to that.

### The solution

What if we used TeX/LaTeX only as an intermediate step when building a document? It would function like an *assembly language* of sorts, and we could happily live in a world of a *higher level* markup language. Doing this, we'd gain the simplicity of Markdown, while still wielding the power of centuries of typesetting expertise that is TeX/LaTeX.

At it's core, this is the solution MDoc proposes as a way into the modern world. Through the help of the incredible document converter [Pandoc](https://pandoc.org/) and the amazing TeX/LaTeX engine made by the [Tectonic](https://github.com/tectonic-typesetting/tectonic) team, we have a solid foundation. MDoc also provides tooling for handling all the nitty gritty of maintaining bigger documents - like bibliography handling, metadata and styling.
