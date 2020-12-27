import os


def make_project(directory):
    os.mkdir(os.path.join(directory, "src"))
    os.mkdir(os.path.join(directory, "assets"))

    with open(os.path.join(directory, "meta.yaml"), "w") as f:
        f.write("title: \"TITLE\"\nauthor: \"AUTHOR\"\ndate: \\today\n\n# Reference equations with parentheses, e.g. '(1)'\neqnos-eqref: True")