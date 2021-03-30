import os
import sys
import re
import subprocess
import shutil

def find_root():
    from kodb import MSG
    root_path = os.getcwd()
    for _ in range(8):
        if os.path.isfile(os.path.join(root_path, "kodb.yaml")):
            return root_path
        root_path = os.path.dirname(root_path)

    MSG.error(f"Could not find the root of a project. Are you situated in a KODB project containing a {style('kodb.yaml', 'bold')} file?")
    sys.exit()


def program_exists(program):
    for path in os.environ["PATH"].split(os.pathsep):
        program_path = os.path.join(path, program)
        if os.path.isfile(program_path) and os.access(program_path, os.X_OK):
            return True
    return False


def find_project_title():
    yaml_path = os.path.join(find_root(), "kodb.yaml")
    with open(yaml_path) as f:
        yaml = f.read()
        project_title = re.search(r"^title: (.*)", yaml, re.M).group(1).replace('"', "")

    return project_title


def find_section(section):
    src_path = os.path.join(find_root(), "src")

    for file in os.listdir(src_path):
        sec = file.split("_")
        sec_index, sec_filename = (int(sec[0]), "_".join(sec[1:]).replace(".md", ""))

        try:
            match = int(section) == sec_index
        except ValueError:
            match = section == sec_filename

        if match: yield os.path.join(src_path, file)


def style(text, *styles):
    code = {
        'red': '31',
        'green': '32',
        'yellow': '33',
        'blue': '34',
        'magenta': '35',
        'cyan': '36',
        'bright red': '91',
        'bright green': '92',
        'bright yellow': '93',
        'bright blue': '94',
        'bright magenta': '95',
        'bright cyan': '96',
        'bold': '1',
        'faint': '2',
        'italic': '3',
        'underline': '4',
        'blink': '5',
        'strike': '9'
    }

    for style in styles:
        text = "\033[" + code[style] + "m" + text + "\033[0m"

    return text


def hr(text=""):
    width = shutil.get_terminal_size()[0]
    ruler = "-" * ((width - len(text)) // 2)
    print(ruler + text + ruler)


def execute(command):
    from kodb import MSG
    try:
        subprocess.run(command, check=True)
    except subprocess.CalledProcessError as e:
        MSG.error(f"Command resulted in error: {style(' '.join(command), 'bold')}.")
        sys.exit()


def right_align(text, left_align_len=0):
    columns = shutil.get_terminal_size()[0]
    if left_align_len + len(text) < columns:
        return(text.rjust(columns))
