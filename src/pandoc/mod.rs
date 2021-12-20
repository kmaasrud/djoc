mod lua;

use anyhow::Result;
use crate::{config::Config, bib};
use std::path::Path;

pub fn make_pandoc_args(config: &Config, root: Option<impl AsRef<Path>>) -> Result<Vec<String>> {
    let mut args = Vec::new();

    // Metadata
    args.push(format!("--metadata=title:{}", config.title));
    args.push(format!("--metadata=date:{}", config.date()));
        
    for author in config.authors.iter() {
        args.push(format!("--metadata=author:{}", author));
    }

    if config.style.number_sections {
        args.push("--number-sections".to_owned());
    }

    // Lua filters
    for filter in lua::get_filters()?.iter().filter_map(|f| f.to_str()) {
        args.push(format!("--lua-filter={}", filter))
    }

    // Bibliography files
    for bib_file in bib::get_bib_files(root)
        .iter()
        .filter_map(|b| b.to_str())
    {
        args.push(format!("--bibliography={}", bib_file));
    }

    // LaTeX options
    // args.push(format!("--metadata=header-includes:{}", config.latex.head));

    // CSL style
    let csl_path = bib::get_csl(&config.bib.csl)?;
    args.push("--csl".to_owned());
    args.push(csl_path.to_string_lossy().to_string());

    args.push("-C".to_owned()); // Use citeproc
    args.push("--metadata=link-citations".to_owned()); // Link to citations (TODO: Make this optional)
    args.push("-s".to_owned());
    args.push("--from=markdown".to_owned());
    args.push("--to=latex".to_owned());

    Ok(args)
}
