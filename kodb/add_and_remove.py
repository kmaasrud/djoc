import os
import sys
from kodb.utils import find_root, find_section, style
from kodb import MSG

ADD_USAGE = f"""Usage:
    {style('kodb add <section name> <section position (optional)>', 'bold')}"""

REMOVE_USAGE = f"""Usage:
    {style('kodb remove <section name or index>', 'bold')}"""


def add_section(name, index=None):
    src_path = os.path.join(find_root(), "src")

    src_files = []
    for file in os.listdir(src_path):
        sec = file.split("_")
        src_files.append({"index": int(sec[0]), "name": "_".join(sec[1:]), "path": os.path.join(src_path, file)})

    src_files.sort(key=lambda x: x["index"])

    if index:
        try:
            index = int(index)
        except ValueError:
            MSG.error(f"Cannot parse the supplied section position {style(index, 'bold')}. The section position must be a parsable integer.")
            sys.exit()

        for file in src_files:
            if file["index"] >= index:
                new_filename = os.path.join(src_path, str(file["index"] + 1).zfill(2) + "_" + file["name"])
                os.rename(file["path"], new_filename)
                file["path"] = new_filename

        new_i = index
    else:
        new_i = 0
        for i, file in enumerate(src_files):
            if i != file["index"]:
                new_i = i
                break
            new_i = i + 1

    new_path = os.path.join(src_path, str(new_i).zfill(2) + "_" + name + ".md")

    if not os.path.isfile(new_path):
        with open(new_path, "w") as f:
            if name.lower() == "abstract":
                f.write("\\begin{abstract}\n\n\\end{abstract}")
            elif name.lower() == "appendix":
                f.write("\\clearpage\n\\appendix\n\n# Appendix\n\n")
            else:
                f.write(f"# {name.capitalize()}\n\n")


def remove_section(sec):
    src_path = os.path.join(find_root(), "src")
    remove_path = list(find_section(sec))

    # If multiple files are matching, enter interactive selection mode.
    if len(remove_path) > 1:
        MSG.warning(f"Found {len(remove_path)} files matching your query, which do you want to remove?")
        print()
        for i, path in enumerate(remove_path):
            print(f"\t{style('(' + str(i) + ')', 'faint')} - {style(path, 'bold')}")
        print()

        wrong_input = True
        while wrong_input:
            selection = input(style("Select a number: ", "bold"))
            try:
                remove_path = remove_path[int(selection)]
            except ValueError:
                MSG.error("Cannot parse the input. Please input a valid integer.")
                continue
            except IndexError:
                MSG.error("Not a valid selection.")
                continue
            wrong_input = False
    else:
        remove_path = remove_path[0]

    renumber = False
    for file in sorted(os.listdir(src_path)):
        if renumber:
            sec = file.split("_")
            new_filename = os.path.join(src_path, str(int(sec[0]) - 1).zfill(2) + "_" + "_".join(sec[1:]))
            os.rename(os.path.join(src_path, file), new_filename)
            continue
        if os.path.join(src_path, file) == remove_path:
            confirmed = input(f"[{style('W', 'bold', 'yellow')}]: Are you sure you want to remove {style(remove_path, 'bold')}? (y/N) ").lower() == 'y'
            if confirmed:
                os.remove(remove_path)
                MSG.success(f"{style(remove_path, 'bold')} removed!")
                renumber = True
                MSG.info("Renumbering sections...")
            else:
                MSG.info("Nothing was deleted.")
                break
