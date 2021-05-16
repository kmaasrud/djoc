---
title: Doctor's bibliography tool
---

# Citation Style Language {#csl}

Doctor comes with a few common CSL styles prepackaged. To use these, just apply them like `csl = "<csl name>"` in the `doctor.toml` file and Doctor will automatically add it to the `assets` directory. Your options are

- [American Psychological Associtaton 6th editon](https://csl.mendeley.com/styleInfo/?styleId=http%3A%2F%2Fwww.zotero.org%2Fstyles%2Fapa), which you can use with `csl = "apa"`.
- [IEEE](https://csl.mendeley.com/styleInfo/?styleId=http%3A%2F%2Fwww.zotero.org%2Fstyles%2Fieee), which you can use with `csl = "ieee"`.
- [Cite Them Right 10th edition - Harvard](https://csl.mendeley.com/styleInfo/?styleId=http%3A%2F%2Fwww.zotero.org%2Fstyles%2Fharvard-cite-them-right), which you can use with `csl = "harvard"`.
- [Nature](https://csl.mendeley.com/styleInfo/?styleId=http%3A%2F%2Fwww.zotero.org%2Fstyles%2Fnature), which you can use with `csl = "nature"`.
- [Vancouver](https://csl.mendeley.com/styleInfo/?styleId=http%3A%2F%2Fwww.zotero.org%2Fstyles%2Fvancouver), which you can use with `csl = "vancouver"`.

For more CSL styles, a good source is the [Mendeley CSL database](https://csl.mendeley.com/searchByName/). There you can find a style you like, copy the URL from the Install button and put it as the CSL style like `csl = "<url>"`. Doctor will then fetch the style from the internet when building your document.

