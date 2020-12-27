import subprocess
import os

def build_document():
    command = ["pandoc"]
    # Make a self-contained TeX document
    command.append("--self-contained")

    # Find all markdown files in 'src' directory
    for f in os.listdir("src"):
        path = os.path.join("src", f)
        if os.path.isfile(path) and os.path.splitext(f)[-1] == ".md":
            command.append(path)
            
    # Convert to TeX
    command.append("-o")
    command.append("main.tex")

    # Use metadatafile if it exists
    if os.path.isfile("meta.yaml"):
        command.append("--metadata-file=meta.yaml")
        
    # Use pandoc-xnos
    command.append("--filter")
    command.append("pandoc-xnos")

    subprocess.call(command)
    
    # Compile outputted TeX document with tectonic and remove the TeX source
    subprocess.call(["tectonic", "main.tex"])
    os.remove("main.tex")