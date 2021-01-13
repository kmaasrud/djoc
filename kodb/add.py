import os
from kodb.utils import find_root


def add_section(name, index=None):
    # Find the kodb project root by searching for a directory containing 'src'
    # This might fail if multiple 'src' directories exist in the project, but since
    # this tool is opinionated, and I keep my code and document source separated, this
    # should be fine.
    root_path = find_root()
        
    markdown_secs = []
    # Loop over existing sections and add to list
    if index:
        for file in os.listdir(os.path.join(root_path, "src")):
            sec = file.split("_")
            sec_index, sec_filename = (sec[0], "_".join(sec[1:]))
            # If file has index higher than the one we want to insert, increase its index by one
            if int(sec_index) >= int(index):
                new_filename = os.path.join(root_path, "src", str(int(sec_index) + 1).zfill(2) + "_" + sec_filename)
                os.rename(os.path.join(root_path, "src", file), new_filename)
        
    new_i = index if index else len(os.listdir(os.path.join(root_path, "src")))
    new_path = os.path.join(root_path, "src", str(new_i).zfill(2) + "_" + name + ".md")
    
    if not os.path.isfile(new_path):
        with open(new_path, "w") as f: f.write(f"# {name.capitalize()}\n\n")