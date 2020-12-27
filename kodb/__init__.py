import sys
from kodb.download import check_program_availability, download_dependencies
from kodb.build import build_document

def main():
    if len(sys.argv) == 1:
        check_program_availability()
        help()

    elif sys.argv[1] in ["--help", "-h"]:
        help()

    elif sys.argv[1] == "build":
        check_program_availability()
        build_document()

    elif sys.argv[1] == "init":
        pass

    elif sys.argv[1] == "new":
        pass

    elif sys.argv[1] == "--download-dependencies":
        download_dependencies()
        
def help():
    print("""Welcome to kodb!""")