import os
from kodb.references import change_style

initial_yaml = """title: "TITLE"
author: "AUTHOR"
date: \\today

# Reference equations with parentheses, e.g. '(1)'
eqnos-eqref: True

# Bibliography
reference-section-title: "References"
bibliography: references.bib
"""

def make_project(directory):
    os.mkdir(os.path.join(directory, "src"))
    os.mkdir(os.path.join(directory, "assets"))

    with open(os.path.join(directory, "kodb.yaml"), "w") as f:
        f.write(initial_yaml)

    with open(os.path.join(directory, "references.bib"), "w") as f:
        f.write("")

    change_style("ieee")
