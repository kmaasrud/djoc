import sys
import os

from kodb.download import check_program_availability, download_dependencies
from kodb.make_project import make_project


def main():
    if len(sys.argv) == 1:
        check_program_availability()
        help()

    elif sys.argv[1] in ["--help", "-h"]:
        help()

    elif sys.argv[1] == "build":
        from kodb.build import build_document
        check_program_availability()
        build_document()

    elif sys.argv[1] == "init":
        make_project(".")

    elif sys.argv[1] == "new":
        try:
            os.mkdir(sys.argv[2])
            make_project(sys.argv[2])
        except IndexError:
            print("A directory name is required as an argument. Run this command like 'kodb new <name>'.")
            
    elif sys.argv[1] == "add":
        from kodb.add import add_section
        try:
            add_section(sys.argv[2], sys.argv[3])
        except IndexError:
            try:
                add_section(sys.argv[2])
            except IndexError:
                print("Add the name of the section you want to add. Run this command like 'kodb add <section name>'")
                
    elif sys.argv[1] == "switch":
        from kodb.switch_and_move import switch_sections
        try:
            switch_sections(sys.argv[2], sys.argv[3])
        except IndexError:
            print("To switch the position of two sections, please include the index or name of the two sections you want to switch place.")
            
    elif sys.argv[1] == "move":
        pass

    elif sys.argv[1] == "--download-dependencies":
        download_dependencies()
        
    elif sys.argv[1]:
        print(f"'{sys.argv[1]}' is not a recognized command, run 'kodb --help' for instructions.")
        

def help():
    print("""Welcome to kodb, a tool which will help you build documents quickly and easily!

To start, create a document in the current directory with 'kodb init' or create a project directory with 'kodb new <project name>.'""")
