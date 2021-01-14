import os
import sys
from kodb.utils import find_root, find_section, style

add_usage = f"""Usage:
    {style('kodb add <section name> <section position (optional)>', 'bold')}"""
    
remove_usage = f"""Usage:
    {style('kodb remove <section name or index>', 'bold')}"""


def add_section(name, index=None):
    src_path = os.path.join(find_root(), "src")

    src_files = []
    for file in os.listdir(src_path):
        sec = file.split("_")
        src_files.append({"index": int(sec[0]), "name": "_".join(sec[1:]), "path": os.path.join(src_path, file)})
        
    src_files.sort(key=lambda x: x["index"])
        
    if index:
        try:
            index = int(index)
        except ValueError:
            print(f"{style('ERROR', ['red', 'bold'])}: The optional argument <section position> must be a parsable integer.")
            print("\n" + add_usage)
            sys.exit()

        for file in src_files:
            if file["index"] >= index:
                new_filename = os.path.join(src_path, str(int(file["index"]) + 1).zfill(2) + "_" + file["name"])
                os.rename(file["path"], new_filename)
                file["path"] = new_filename
                
        new_i = index
    else:
        new_i = 0
        for i, file in enumerate(src_files):
            if i != file["index"]:
                new_i = i
                break
            new_i = i + 1
        
    new_path = os.path.join(src_path, str(new_i).zfill(2) + "_" + name + ".md")
    
    if not os.path.isfile(new_path):
        with open(new_path, "w") as f:
            if name.lower() == "abstract":
                f.write("\\begin{abstract}\n\n\\end{abstract}") 
            elif name.lower() == "appendix":
                f.write("\\clearpage\n\\appendix\n")
            else:
                f.write(f"# {name.capitalize()}\n\n")
                
def remove_section(sec):
    sec_path = find_section(sec)
    os.remove(sec_path)

