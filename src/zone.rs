use anyhow::{anyhow, Result};

pub fn parse_zone(input: &str) -> Result<u8> {
    let cleaned = input.to_lowercase().replace("zone", "");
    let zone = cleaned
        .trim()
        .parse()
        .map_err(|_| anyhow!("Invalid zone"))?;
    if zone > 3 {
        return Err(anyhow!("Zone much be 0-3"));
    }
    Ok(zone)
}
