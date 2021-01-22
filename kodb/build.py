import subprocess
import os
import sys
from kodb.utils import find_root

def build_document():
    root_path = find_root()

    command = ["pandoc"]
    # Make a self-contained TeX document
    command.append("--self-contained")

    # Find all markdown files in 'src' directory
    src_files = []
    for f in os.listdir(os.path.join(root_path, "src")):
        path = os.path.join(root_path, "src", f)
        if os.path.isfile(path) and os.path.splitext(f)[-1] == ".md":
            src_files.append(path)
    command += sorted(src_files, key=lambda x: x.split("_")[0].split(os.sep)[-1])
            
    # Convert to TeX
    command.append("-o")
    command.append(os.path.join(root_path, "main.tex"))

    # Use metadatafile if it exists
    if os.path.isfile(os.path.join(root_path, "kodb.yaml")):
        command.append(f"--metadata-file={os.path.join(root_path, 'kodb.yaml')}")
        
    # Use pandoc-xnos
    command.append("--filter")
    command.append("pandoc-xnos")
    
    # Use citeproc. References are placed in the YAML file
    command.append("--citeproc")

    subprocess.call(command)
    
    # Compile outputted TeX document with tectonic and remove the TeX source
    subprocess.call(["tectonic", os.path.join(root_path, "main.tex")])
    os.remove(os.path.join(root_path, "main.tex"))
