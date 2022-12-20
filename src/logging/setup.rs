use anyhow::Result;

pub fn setup_logging(path: &str) -> Result<()> {
    log4rs::init_file(path, Default::default())?;
    Ok(())
}
