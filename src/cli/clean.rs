use std::io;

use log::info;

pub fn clean() -> io::Result<()> {
    info!("Cleaning up the working directory...");
    let path = std::env::current_dir()?;
    std::fs::remove_dir_all(path.join("build"))?;
    Ok(())
}
