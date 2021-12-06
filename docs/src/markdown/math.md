## Inline math

Anything between two `$` characters will be treated as $\TeX$ math. The opening \$ must have a non-space character immediately to its right, while the closing \$ must have a non-space character immediately to its left, and must not be followed immediately by a digit. Thus, `$20,000` and `$30,000` won’t parse as math. If for some reason you need to enclose text in literal \$ characters, escape them with a blackslash and they won’t be treated as math delimiters.

```markdown
Here is some text and here is some math: $E = mc^2$.
```

## Display math

For display math, use `$$` delimiters. In this case, the delimiters may be separated from the formula by whitespace. However, there can be no blank lines betwen the opening and closing delimiters.

```markdown
And God said

$$ \nabla \cdot \vec D = \rho_\nu, $$
$$ \nabla \cdot \vec B = 0, $$
$$ \nabla \times \vec E = -\frac{\partial \vec B}{\partial t}, $$
$$ \nabla \times \vec H = \vec J_\text{free} + \frac{\partial \vec D}{\partial t}, $$

and then there was light.
```

and then there was light.
