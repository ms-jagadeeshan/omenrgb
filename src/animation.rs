use anyhow::{Result, anyhow};

use crate::constants::MODES;

pub fn validate_mode(input: &str) -> Result<()> {
    if MODES.contains(&input) {
        Ok(())
    } else {
        Err(anyhow!("Invalid animation mode"))
    }
}
