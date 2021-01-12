import os

def find_root():
    root_path = os.getcwd()
    for _ in range(8):
        if os.path.isfile(os.path.join(root_path, "kodb.yaml")):
            return root_path
        root_path = os.path.dirname(root_path)
        
    raise FileNotFoundError
