import os

def find_root():
    root_path = os.getcwd()
    for _ in range(8):
        if os.path.isdir(os.path.join(root_path, "src")):
            return root_path
        root_path = os.path.dirname(root_path)
        
    raise FileNotFoundError