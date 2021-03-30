import os
import re
import requests
from kodb.utils import find_root, style
from kodb import MSG

STYLE_LOOKUP = {
        "ieee": { "filename": "ieee.csl", "url": "https://raw.githubusercontent.com/citation-style-language/styles/master/ieee.csl" },
        "apa": { "filename": "apa-6th-edition.csl", "url": "https://raw.githubusercontent.com/citation-style-language/styles/master/apa-6th-edition.csl" }
}

def change_style(stylename):
    root = find_root()

    try:
        csl_name = STYLE_LOOKUP[stylename]["filename"]
    except KeyError:
        MSG.error(f"Could not find a CSL style matching the query {style(stylename, 'bold')}")
        return
    MSG.success(f"Found a matching CSL file for {style(stylename, 'bold')}!")

    if not os.path.isfile(os.path.join(root, "assets", csl_name)):
        MSG.info(f"Downloading {style(csl_name, 'bold')} from {style(STYLE_LOOKUP[stylename]['url'], 'italic')}...")
        try:
            resp = requests.get(STYLE_LOOKUP[stylename]["url"])
        except requests.exceptions.ConnectionError:
            MSG.error(f"You need to be connected to the internet to download the CSL file {style(csl_name, 'bold')}.")
            return
        MSG.success(f"{style(csl_name, 'bold')} successfully downloaded!")
        if resp.ok:
            csl = resp.text
        else:
            MSG.error(f"Invalid response. Could not download {style(csl_name, 'bold')} from the internet.")
            return
        with open(os.path.join(root, "assets", csl_name), "w") as f:
            f.write(csl)
    else:
        MSG.info(f"Found {style(csl_name, 'bold')} in assets directory.")


    with open(os.path.join(root, "kodb.yaml"), "r+") as f:
        kodb_yaml = f.read()

        if match := re.search(r"^csl: (.*)\s*$", kodb_yaml, re.M):
            MSG.info(f"Changing CSL style in {style('kodb.yaml', 'bold')}...")
            kodb_yaml = kodb_yaml.replace(f"{match.group()}", f"csl: {csl_name}")
        else:
            MSG.info(f"Adding CSL style specification to {style('kodb.yaml', 'bold')}...")
            kodb_yaml += "\n" + f"csl: {csl_name}"

        f.seek(0)
        f.write(kodb_yaml)
        f.truncate()

    MSG.success("Reference style successfully updated!")
