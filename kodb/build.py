import subprocess
import os
import shutil
import sys
import re
from kodb.utils import find_root, execute, hr, style
from kodb import MSG


def build_document():
    root_path = find_root()

    command = ["pandoc"]
    # Make a self-contained .tex document
    command += ["--self-contained", "-s"]

    # Find all markdown files in 'src' directory
    MSG.info("Finding source files...")
    src_files = []
    for f in os.listdir(os.path.join(root_path, "src")):
        path = os.path.join(root_path, "src", f)
        if os.path.isfile(path) and os.path.splitext(f)[-1] == ".md":
            # Append a few newlines to ensure proper headers
            with open(path, "a+") as f:
                if re.search(r"\n\n\Z", f.read(), re.MULTILINE):
                    f.write("\n\n")

            src_files.append(path)

    # Sort by the file numbers
    command += sorted(src_files, key=lambda x: x.split("_")[0].split(os.sep)[-1])
    MSG.success(f"{len(src_files)} files found!")

    # Resources can be fetched from both root, assets and src. They are extracted into tmp for linking with main.tex
    pathsep = ';' if os.name == 'nt' else ':'
    command.append(f"--resource-path={root_path}{pathsep}{os.path.join(root_path, 'src')}{pathsep}{os.path.join(root_path, 'assets')}")
    command.append(f"--extract-media={os.path.join(root_path, 'tmp')}")

    # Convert to TeX
    command.append("-o")
    command.append(os.path.join(root_path, "main.tex"))

    # Use metadatafile if it exists
    if os.path.isfile(yaml := os.path.join(root_path, "kodb.yaml")):
        command.append(f"--metadata-file={yaml}")

    # Use pandoc-xnos
    command += ["--filter", "pandoc-xnos"]

    # Use citeproc
    command.append("--citeproc")

    hr("Pandoc")
    MSG.info("Converting the Markdown source into LaTeX via Pandoc...")
    execute(command)
    MSG.success(f"{style('main.tex', 'bold')} created!")

    # Compile document with Tectonic. Only output necessary warnings and errors.
    command = ["tectonic", "--chatter", "minimal", os.path.join(root_path, "main.tex")]

    hr("Tectonic")
    MSG.info("Compiling document with Tectonic...")
    execute(command)
    MSG.success("Successfully compiled the PDF!")
    hr()

    try:
        shutil.rmtree(os.path.join(root_path, 'tmp'))
        MSG.info(f"Removed temporary directory: {style(os.path.join(root_path, 'tmp'), 'bold')}")
    except FileNotFoundError:
        pass
    os.remove(os.path.join(root_path, "main.tex"))
    MSG.info(f"Removed temporary TeX file: {style(os.path.join(root_path, 'main.tex'), 'bold')}")
