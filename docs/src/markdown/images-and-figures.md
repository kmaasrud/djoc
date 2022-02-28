A link immediately preceded by a `!` will be treated as an image.

```markdown
![](image-path.jpg)

![movie reel]

[movie reel]: movie.gif

![figure caption](another-image.png)
```

As you can see, you set the caption of the figure within the link text field. The attribute syntax is also supported on images, meaning you can define identifiers, classes and key-value pairs on images as well.

```markdown
An inline ![image](foo.jpg){#id .class width=30 height=20px}
and a reference ![image][ref] with attributes.

[ref]: foo.jpg "optional title" {#id .class key=val key2="val 2"}
```

### Sizing

An image can be resized by utilizing the `width` and `height` keys. Say you want to insert an image, but make it half as big.

```markdown
![](file.jpg){ width=50% }
```

`100%` is defined by the text width, meaning the above will output a centered image that has half the width of the surrounding text.
