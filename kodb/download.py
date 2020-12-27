import os
import subprocess
import requests
import tarfile

def program_exists(program):
    for path in os.environ["PATH"].split(os.pathsep):
        program_path = os.path.join(path, program)
        if os.path.isfile(program_path) and os.access(program_path, os.X_OK):
            return True
        
    print(f"{program} not found in PATH.")
    return False

def download_tectonic():
    print("Downloading tectonic AppImage 0.3.3 from GitHub...")
    if os.name == "nt":
        pass #TODO
    else:
        r = requests.get("https://github.com/tectonic-typesetting/tectonic/releases/download/tectonic%400.3.3/tectonic-0.3.3-x86_64.AppImage")

        exe_path = "/usr/bin/tectonic"
        with open(exe_path, "wb") as f:
            f.write(r.content)

        mode = os.stat(exe_path).st_mode
        mode |= (mode & 0o444) >> 2
        os.chmod(exe_path, mode)

    print("Tectonic downloaded!")
    
def download_pandoc():
    print("Downloading pandoc 2.11.3.1 from GitHub...")
    if os.name == "nt":
        pass #TODO
    else:
        r = requests.get("https://github.com/jgm/pandoc/releases/download/2.11.3.1/pandoc-2.11.3.1-linux-amd64.tar.gz")
        with open("pandoc_temp.tar.gz", "wb") as f:
            f.write(r.content)
        with tarfile.open("pandoc_temp.tar.gz", "r:gz") as f:
            f.extractall()
    
    print("Pandoc downloaded!")

def ensure_program_availability():
    if not program_exists("pandoc"):
        download_pandoc()
    if not program_exists("tectonic"):
        download_tectonic()