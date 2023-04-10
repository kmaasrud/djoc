use std::io;

use djoc::utils::find_root;
use log::info;

pub fn clean() -> io::Result<()> {
    info!("Cleaning up the working directory...");
    let path = find_root(std::env::current_dir()?);
    std::fs::remove_dir_all(path.join("build"))?;
    Ok(())
}
