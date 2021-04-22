source := src
output := out
sources := $(shell find src/ -type f -name '*.md')
objects := $(addprefix out/,$(patsubst %.md,%.html,$(notdir $(sources))))

all: $(objects)

$(output)/%.html: src/%.md
	pandoc $< -s -o $@ --template=template.html --katex --toc --shift-heading-level-by=1
$(output)/%.html: src/markdown/%.md
	pandoc $< -s -o $@ --template=template.html --katex --toc --shift-heading-level-by=1 --no-highlight

.PHONY: clean

clean:
	rm -f $(output)/*.html
# pandoc:
# 	pandoc creating-your-first-document.md -s -o out/creating-your-first-document.html --template=template.html --katex --toc
# 	pandoc installation.md -s -o out/installation.html --template=template.html --katex --toc
# 	pandoc commands.md -s -o out/commands.html --template=template.html --katex --toc
