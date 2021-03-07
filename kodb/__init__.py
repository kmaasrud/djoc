import sys
import os

from .log import Message
MSG = Message("INFO")
from .download import check_program_availability, download_dependencies
from .make_project import make_project, make_dir_wrapper
from .utils import style


def build(_):
    from kodb.build import build_document
    check_program_availability()
    build_document()


def init(_):
    make_project(".")


def new(args):
    if not args:
        MSG.error(f"A directory name is required as an argument. Run this command like {style('kodb new <name>', 'bold')}.")
        sys.exit()
    make_dir_wrapper(args[0])
    make_project(args[0])


def add(args):
    from kodb.add_and_remove import add_section
    if not args:
        print("Add the name of the section you want to add. Run this command like 'kodb add <section name>'")
        sys.exit()
    elif len(args) == 1:
        add_section(args[0])
    else:
        add_section(args[0], args[1])


def switch(args):
    if not len(args) == 2:
        print("To switch the position of two sections, please include the index or name of the two sections you want to switch place.")
        sys.exit()
    from kodb.switch_and_move import switch_sections
    switch_sections(args[0], args[1])


def move(args):
    pass


def remove(args):
    from kodb.add_and_remove import remove_section
    for section in args:
        remove_section(section)


def edit(args):
    from kodb.edit_and_list import edit_project
    if args:
        edit_project(args[0])
    else:
        edit_project(args)


def list(args):
    from kodb.edit_and_list import list_sections
    list_sections()


def ref(args):
    from kodb.references import change_style
    change_style(args[0])


def default_doc_structure(_):
    from kodb.add_and_remove import add_section
    for section in ["abstract", "introduction", "theory", "results", "discussion", "conclusion", "references", "appendix"]:
        add_section(section)


def help(_):
    print(f"""Welcome to kodb, a tool which will help you build documents quickly and easily!

To start, create a document in the current directory with {style('kodb init', 'bold')} or create a project directory with {style('kodb new <project name>.', 'bold')}""")


command_lookup = {
    "build": build,
    "init": init,
    "new": new,
    "add": add,
    "switch": switch,
    "move": move,
    "remove": remove,
    "edit": edit,
    "list": list,
    "ref": ref,
    "help": help,
    "-h": help,
    "--help": help,
    "--download-dependencies": download_dependencies,
    "--default-doc-structure": default_doc_structure
}


def main():
    args = sys.argv
    if len(args) == 1:
        help(args)
        sys.exit()
    elif len(args) == 2:
        command = args[1]
        command_args = None
    else:
        command = args[1]
        command_args = args[2:]

    try:
        command_lookup[command](command_args)
    except KeyError:
        MSG.error(f"Unknown command \"{command}\"")
