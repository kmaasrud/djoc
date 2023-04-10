use std::{io::Read, path::PathBuf};

use anyhow::{anyhow, Result};
use log::info;
use url::Url;

use crate::utils::{data_dir, find_root};

pub fn get_csl(id: &str) -> Result<PathBuf> {
    let path = find_root(".").join(id);
    if path.exists() {
        return Ok(path);
    }

    let url = if let Ok(url) = Url::parse(id) {
        // id is parsable URL, so use that URL
        url
    } else {
        // id is the filestem of a CSL file contained in the
        // citation-style-language/styles repo
        Url::parse("https://raw.githubusercontent.com/citation-style-language/styles/master/")?
            .join(&format!("{id}.csl"))?
    };

    let filename = url
        .path_segments()
        .ok_or_else(|| anyhow!("Could not determine the segments of \"{url}\".\nDoes your URL point to a valid CSL file?"))?
        .last()
        .ok_or_else(|| anyhow!("Unable to find the filename of \"{url}\".\nDoes your URL point to a valid CSL file?"))?;

    let path = data_dir().join("csl").join(filename).with_extension("csl");

    if !path.exists() {
        // If a cached CSL file doesn't exist, download it
        info!(
            "Fetching {:?} from \"{url}\" ...",
            path.file_name().unwrap()
        );
        let resp = ureq::get(url.as_str()).call()?;

        let mut bytes = Vec::new();
        resp.into_reader()
            .take(10_000_000)
            .read_to_end(&mut bytes)?;

        std::fs::write(&path, bytes)?;
    }

    Ok(path)
}
