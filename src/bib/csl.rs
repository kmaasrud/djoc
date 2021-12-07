use anyhow::{anyhow, Result};
use crate::utils::{data_dir, write_file};
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

    let filename = url.path_segments().unwrap().last().ok_or_else(|| anyhow!("blabla"))?;
    let path = data_dir().join("csl").join(filename).with_extension("csl");
    if !path.exists() {
        info!("Fetching \"{}\" from \"{}\"...", filename, url);
        let resp = ureq::get(url.as_str()).call()?;

        let mut bytes = Vec::new();
        resp.into_reader()
            .take(10_000_000)
            .read_to_end(&mut bytes)?;

        write_file(&path, &bytes)?;
    }

    Ok(path)
}
