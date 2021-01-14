import os

def find_root():
    root_path = os.getcwd()
    for _ in range(8):
        if os.path.isfile(os.path.join(root_path, "kodb.yaml")):
            return root_path
        root_path = os.path.dirname(root_path)
        
    raise FileNotFoundError

def cwd_is_proj():
    try:
        find_root()
        return True
    except FileNotFoundError:
        return False
    
def style(text, color):
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

    return '\033[' + code[color] + 'm' + text + '\033[0m'