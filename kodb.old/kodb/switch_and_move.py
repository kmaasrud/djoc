import os
from kodb.utils import find_root


def switch_sections(sec1, sec2):
    src_path = os.path.join(find_root(), "src")
    file1, file2 = find(src_path, [sec1, sec2])
    file1_old = os.path.join(src_path, "_".join(file1))
    file2_old = os.path.join(src_path, "_".join(file2))
    file1_new = os.path.join(src_path, file2[0] + "_" + file1[1])
    file2_new = os.path.join(src_path, file1[0] + "_" + file2[1])
    os.rename(file1_old, file1_new)
    os.rename(file2_old, file2_new)
                    

def find(src_path, not_found, found=[]):
    for file in os.listdir(src_path):
        sec = file.split("_")
        sec_index, sec_filename = (int(sec[0]), "_".join(sec[1:]).replace(".md", ""))

        try:
            match = int(not_found[0]) == sec_index
        except ValueError:
            match = not_found[0] == sec_filename
            
        if match:
            file = (file.split("_")[0], "_".join(file.split("_")[1:]))
            if found:
                return found, file
            return find(src_path, not_found[1:], found=file)