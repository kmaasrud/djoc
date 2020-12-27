import os
import sys
import requests
import tarfile
from math import prod


def program_exists(program):
    for path in os.environ["PATH"].split(os.pathsep):
        program_path = os.path.join(path, program)
        if os.path.isfile(program_path) and os.access(program_path, os.X_OK):
            return True
    return False


def download_tectonic():
    TECTONIC_VER = "0.3.3"
    print(f"Downloading tectonic AppImage {TECTONIC_VER} from GitHub...")

    if os.name == "nt":
        pass #TODO
    else:
        r = requests.get(f"https://github.com/tectonic-typesetting/tectonic/releases/download/tectonic%40{TECTONIC_VER}/tectonic-{TECTONIC_VER}-x86_64.AppImage")

        exe_path = "/usr/bin/tectonic"
        with open(exe_path, "wb") as f:
            f.write(r.content)

        mode = os.stat(exe_path).st_mode
        mode |= (mode & 0o444) >> 2
        os.chmod(exe_path, mode)

    print("Tectonic downloaded!")
    

def download_pandoc():
    PANDOC_VER = "2.11.3.1"
    print(f"Downloading pandoc {PANDOC_VER} from GitHub...")

    if os.name == "nt":
        pass #TODO
    else:
        r = requests.get(f"https://github.com/jgm/pandoc/releases/download/{PANDOC_VER}/pandoc-{PANDOC_VER}-linux-amd64.tar.gz")
        with open("pandoc_temp.tar.gz", "wb") as f:
            f.write(r.content)
        with tarfile.open("pandoc_temp.tar.gz", "r:gz") as f:
            f.extractall()
        os.rename(f"pandoc-{PANDOC_VER}/bin/pandoc", "/usr/bin/pandoc")
        os.system(f"rm -rf pandoc-{PANDOC_VER}")
        os.remove("pandoc_temp.tar.gz")
    
    print("Pandoc downloaded!")
    
    
def download_pandoc_xnos():
    os.system("pip install pandoc-fignos pandoc-eqnos pandoc-tablenos pandoc-secnos")


def download_dependencies():
    if not program_exists("pandoc"):
        download_pandoc()

    if not program_exists("tectonic"):
        download_tectonic()

    check = prod([program_exists(prog) for prog in [
        "pandoc-xnos", "pandoc-fignos", "pandoc-eqnos", "pandoc-tablenos", "pandoc-secnos"]])
    if not check:
        download_pandoc_xnos()
        

def check_program_availability():
    for prog in ["pandoc", "tectonic", "pandoc-xnos", "pandoc-fignos", "pandoc-eqnos", "pandoc-tablenos", "pandoc-secnos"]:
        if not program_exists(prog):
            print(f"""ERROR: {prog} does not exist on this system or is not in PATH. Preferably download from the original source, or
alternatively run 'sudo kodb --download-dependencies' to install required dependencies (may have varying success).""")
            sys.exit()