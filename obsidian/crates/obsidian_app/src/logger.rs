// crates/obsidian_app/src/logger.rs
use anyhow::{Context, Result};
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::{fs::File, path::Path};

pub fn create_logger(path: impl AsRef<Path>) -> Result<()> {
    let name = path.as_ref().display().to_string();
    let error_message = format!("Failed to create log file named: {}", name);
    let file = File::create(path).context(error_message)?;
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(LevelFilter::max(), Config::default(), file),
    ])?;
    Ok(())
}
