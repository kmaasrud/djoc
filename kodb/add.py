import os


def add_section(name):
    # Find the kodb project root by searching for a directory containing 'src'
    # This might fail if multiple 'src' directories exist in the project, but since
    # this tool is opinionated, and I keep my code and document source separated, this
    # should be fine.
    root_path = os.getcwd()
    for _ in range(8):
        if os.path.isdir(os.path.join(root_path, "src")):
            break
        root_path = os.path.dirname(root_path)
        
    markdown_secs = []
    for i, file in enumerate(os.listdir(os.path.join(root_path, "src"))):
        sec_index, sec_name = file.replace(".md", "").split("_")
        markdown_secs.append({"i": int(sec_index), "name": sec_name, "path": os.path.join(root_path, "src", file)})
        
    new_i = len(markdown_sources)
    new_path = os.path.join(root_path, "src", str(new_i).zfill(2) + "_" + name + ".md")
    markdown_secs.append({"i": new_i, "name": name, "path": new_path})
    
    for sec in markdown_secs:
        create_unless_exists(sec)
    

def create_unless_exists(section_dict):
    if not os.path.isfile(section_dict["path"]):
        with open(section_dict["path"], "w") as f: f.write("")