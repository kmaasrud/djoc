use anyhow::{Context, Result};
use mdoc::utils::data_dir;
use std::fs::remove_dir_all;

pub fn clean(data: bool) -> Result<()> {
    if data {
        remove_dir_all(data_dir()).context("Could not remove data directory.")?;
        mdoc::success!("Data directory removed.");
    } else {
        mdoc::info!("MDoc does not support cleaning the project folder yet.");
    }

    Ok(())
}
