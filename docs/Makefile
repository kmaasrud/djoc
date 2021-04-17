source := src
output := out
sources := $(wildcard $(source)/*.md)

objects := $(patsubst %.md,%.html,$(subst $(source),$(output),$(sources)))

all: $(objects)

$(output)/%.html: $(source)/%.md
	pandoc $< -s -o $@ --template=template.html --katex --toc

.PHONY : clean

clean:
	rm -f $(output)/*.html
# pandoc:
# 	pandoc creating-your-first-document.md -s -o out/creating-your-first-document.html --template=template.html --katex --toc
# 	pandoc installation.md -s -o out/installation.html --template=template.html --katex --toc
# 	pandoc commands.md -s -o out/commands.html --template=template.html --katex --toc
