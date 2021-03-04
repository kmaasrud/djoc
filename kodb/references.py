import os
import re
import requests
from kodb.utils import find_root, style

STYLE_LOOKUP = {
        "ieee": { "filename": "ieee.csl", "url": "https://raw.githubusercontent.com/kmaasrud/vmc-fys4411/main/doc/assets/ieee.csl" }
}

def change_style(stylename):
    root = find_root()

    try:
        csl_name = STYLE_LOOKUP[stylename]["filename"]
    except KeyError:
        print(f"{style('ERROR', 'bold', 'red')}: Could not find a CSL style matching the query \"{stylename}\"")
        return

    try:
        resp = requests.get(STYLE_LOOKUP[stylename]["url"])
    except requests.exceptions.ConnectionError:
        print(f"{style('ERROR', 'bold', 'red')}: You need to be connected to the internet to download the CSL file \"{STYLE_LOOKUP[stylename]['filename']}\"")
        return

    if resp.ok:
        csl = resp.text
    else:
        print(f"{style('ERROR', 'bold', 'yellow')}: Invalid response. Could not download \"{STYLE_LOOKUP[stylename]['filename']}\" from the internet.")
        return

    csl = requests.get(STYLE_LOOKUP[stylename]["url"]).text

    with open(os.path.join(root, "assets", "ieee.csl"), "w") as f:
        f.write(csl)

    with open(os.path.join(root, "kodb.yaml"), "r+") as f:
        kodb_yaml = f.read()

        if match := re.search(r"^csl: (.*)", kodb_yaml, re.M):
            kodb_yaml = kodb_yaml.replace(f"csl: {match.group(1)}", f"csl: assets/{csl_name}")
        else:
            kodb_yaml += "\n" + f"csl: assets/{csl_name}"

        f.seek(0)
        f.write(kodb_yaml)
