# Inline math

Anything between two `$` characters will be treated as $\TeX$ math. The opening `$` must have a non-space character immediately to its right, while the closing `$` must have a non-space character immediately to its left, and must not be followed immediately by a digit. Thus, `$20,000` and `$30,000` won’t parse as math. If for some reason you need to enclose text in literal `$` characters, escape them with a blackslash and they won’t be treated as math delimiters.

```markdown
Here is some text and here is some math: $E = mc^2$.
```

# Display math

For display math, use `$$` delimiters. In this case, the delimiters may be separated from the formula by whitespace. However, there can be no blank lines betwen the opening and closing `$$` delimiters. I also advise you to start and end your equation on the same line as your `$$` signs, as some Pandoc filters work best that way.

```markdown
And God said

$$ \nabla \cdot \vec D = \rho_\nu, $$
$$ \nabla \cdot \vec B = 0, $$
$$ \nabla \times \vec E = -\frac{\partial \vec B}{\partial t}, $$
$$ \nabla \times \vec H = \vec J_\text{free} + \frac{\partial \vec D}{\partial t}, $$

and then there was light.
```

## Cross-referencing

You can add a number to your display math and cross-reference it with the following syntax:

```markdown
$$ f(k) = {n \choose k} p^{k} (1-p)^{n-k} $${#eq:binomial-distribution}

The binomial distribution is shown in [@eq:binomial-distribution].
```

Adding `{#eq:binomial-distribution}` (the `#eq` part is required) after the equation assigns the ID `binomial-distribution` to the equation and automatically numbers it. This number gets inserted into the text by writing `[@eq:binomial-distribution]` (the `@eq:` part is required here too).
