use crate::driver;
use anyhow::Result;
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct Status {
    brightness: u8,
    mode: String,
    speed: u8,
    zones: Vec<String>,
}

pub fn read_status() -> Result<Status> {
    Ok(Status {
        brightness: driver::get_brightness()?,
        mode: driver::get_mode()?,
        speed: driver::get_speed()?,
        zones: (0..4)
            .map(|z| driver::get_zone(z))
            .collect::<Result<Vec<_>, _>>()?,
    })
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Brightness: {}%", self.brightness)?;
        writeln!(f, "Mode: {}", self.mode)?;
        writeln!(f, "Speed: {}%", self.speed)?;
        for (i, z) in self.zones.iter().enumerate() {
            writeln!(f, "Zone {}: {}", i, z)?;
        }
        Ok(())
    }
}
