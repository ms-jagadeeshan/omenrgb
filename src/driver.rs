use anyhow::{Context, Result};
use std::fs;

const BASE: &str = "/sys/devices/platform/omen-rgb-keyboard/rgb_zones/";

fn write(name: &str, value: &str) -> Result<()> {
    fs::write(format!("{BASE}{name}"), value).with_context(|| format!("Failed writing {name}"))
}

fn read(name: &str) -> Result<String> {
    Ok(fs::read_to_string(format!("{BASE}{name}"))?
        .trim()
        .to_string())
}

pub fn set_zone(zone: u8, color: &str) -> Result<()> {
    write(&format!("zone{:02}", zone), color)
}

pub fn set_all(color: &str) -> Result<()> {
    write("all", color)
}

pub fn set_brightness(value: u8) -> Result<()> {
    write("brightness", &value.to_string())
}

pub fn set_mode(mode: &str) -> Result<()> {
    write("animation_mode", mode)
}

pub fn set_speed(value: u8) -> Result<()> {
    write("animation_speed", &value.to_string())
}

pub fn get_brightness() -> Result<u8> {
    Ok(read("brightness")?.parse::<u8>()?)
}

pub fn get_speed() -> Result<u8> {
    Ok(read("animation_speed")?.parse::<u8>()?)
}

pub fn get_mode() -> Result<String> {
    read("animation_mode")
}

pub fn get_zone(zone: u8) -> Result<String> {
    read(&format!("zone{:02}", zone))
}
