import os
import sys
from kodb.references import change_style
from kodb.utils import style
from kodb import MSG

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
    make_dir_wrapper(os.path.join(directory, "src/"))

    make_dir_wrapper(os.path.join(directory, "assets/"))

    path = os.path.join(directory, "kodb.yaml")
    with open(path, "w") as f:
        f.write(initial_yaml)
    MSG.success(f"Created {style(path, 'bold')}.")

    path = os.path.join(directory, "assets", "references.bib")
    with open(path, "w") as f:
        f.write("")
    MSG.success(f"Created {style(path, 'bold')}.")

def make_dir_wrapper(path):
    try:
        os.mkdir(path)
    except FileExistsError:
        MSG.error(f"Directory {style(path, 'bold')} already exists. Aborting...")
        sys.exit()
    MSG.success(f"Created {style(path, 'bold')} directory.")
