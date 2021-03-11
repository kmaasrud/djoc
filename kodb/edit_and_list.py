import os
import re
import subprocess
from kodb.utils import program_exists, find_root, find_section, style, find_project_title, right_align


# Need some way of defining your own preferred editor. Perhaps use $EDITOR? This is not Windows-friendly though...
PREFFERED_EDITORS = ["nvim", "code", "code-insiders", "vim", "vi", "nano"]


def edit_project(section):
    root_path = find_root()

    for editor in PREFFERED_EDITORS:
        if program_exists(editor):
            break

    command = [editor]

    if not section:
        command.append(os.path.join(root_path, "src"))
    elif section.lower() in ["yaml", "kobd.yaml", "kodb"]:
        command.append(os.path.join(root_path, "kodb.yaml"))
    else:
        section_path = next(find_section(section))
        command.append(section_path)

    subprocess.call(command)


def list_sections():
    print()
    print(f"Structure of {style(find_project_title(), 'bold')}:\n")
    src_path = os.path.join(find_root(), "src")
    header_re = re.compile(r"(^(.+)[ \t]*\n(=+|-+)[ \t]*\n+)|(^(\#{1,6})[ \t]+(.+?)[ \t]*(?<!\\)\#*\n+)", re.M)

    title = None
    section_paths = sorted(os.listdir(src_path))
    for i, f in enumerate(section_paths):
        with open(os.path.join(src_path, f)) as file:
            text = file.read()
            match = header_re.search(text)
            if match and (match.group(2) or match.group(6)):
                if match.group(3) and match.group(3)[0] == "=":
                    title = match.group(2)
                elif len(match.group(5)) == 1:
                    title = match.group(6)
                else:
                    title = None

        # If title is not found via the method above, strip the index and extension of the path and use is as title
        title = re.sub(r"\d\d_", "", f).replace(".md", "").capitalize() if not title else title

        print(f"\t{style('(' + str(i) + ')', 'faint')} - {style(title, 'bold')}")

    print()
