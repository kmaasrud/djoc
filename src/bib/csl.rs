use crate::utils::{data_dir, write_file};
use anyhow::{anyhow, Result};
use std::io::Read;
use std::path::PathBuf;
use url::Url;

pub fn get_csl(id: &str) -> Result<PathBuf> {
    let url = if let Ok(url) = Url::parse(id) {
        url
    } else {
        Url::parse("https://raw.githubusercontent.com/citation-style-language/styles/master/")?
            .join(&format!("{}.csl", id))?
    };

    let filename = url
        .path_segments()
        .ok_or_else(|| anyhow!("Could not determine the segments of \"{}\".\nDoes your URL point to a valid CSL file?", url))?
        .last()
        .ok_or_else(|| anyhow!("Unable to find the filename of \"{}\".\nDoes your URL point to a valid CSL file?", url))?;
    let path = data_dir().join("csl").join(filename).with_extension("csl");

    if !path.exists() {
        info!(
            "Fetching {:?} from \"{}\" ...",
            path.file_name().unwrap(),
            url
        );
        let resp = ureq::get(url.as_str()).call()?;

        let mut bytes = Vec::new();
        resp.into_reader()
            .take(10_000_000)
            .read_to_end(&mut bytes)?;

        write_file(&path, &bytes)?;
    }

    Ok(path)
}
