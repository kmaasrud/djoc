import sys
from kodb.utils import program_exists
from kodb import MSG


def check_program_availability():
    for prog in ["pandoc", "tectonic", "pandoc-xnos", "pandoc-fignos", "pandoc-eqnos", "pandoc-tablenos", "pandoc-secnos"]:
        if not program_exists(prog):
            MSG.error(f"{prog} does not exist on this system or is not in PATH.")
            sys.exit()
