use crate::color;
use crate::{cli::PresetCmd, driver};
use anyhow::Result;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Preset {
    brightness: u8,
    mode: String,
    speed: u8,
    zones: Vec<String>,
}

fn preset_path(name: &str) -> std::path::PathBuf {
    config_dir()
        .unwrap()
        .join("omenrgb")
        .join(format!("{name}.json"))
}

pub fn handle_preset(cmd: PresetCmd) -> Result<()> {
    match cmd {
        PresetCmd::Save { name } => save(&name),
        PresetCmd::Load { name } => load(&name),
        PresetCmd::List => list(),
    }
}

fn save(name: &str) -> Result<()> {
    let preset = Preset {
        brightness: driver::get_brightness()?,
        mode: driver::get_mode()?,
        speed: driver::get_speed()?,
        zones: (0..4)
            .map(|z| driver::get_zone(z))
            .collect::<Result<Vec<_>, _>>()?,
    };
    let path = preset_path(name);
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, serde_json::to_string(&preset)?)?;
    Ok(())
}

fn load(name: &str) -> Result<()> {
    let data = fs::read_to_string(preset_path(name))?;
    let preset: Preset = serde_json::from_str(&data)?;

    driver::set_brightness(preset.brightness)?;
    driver::set_mode(&preset.mode)?;
    driver::set_speed(preset.speed)?;
    for (i, color) in preset.zones.iter().enumerate() {
        let color = color::normalize_color(color)?;
        driver::set_zone(i as u8, &color)?;
    }
    Ok(())
}

fn list() -> Result<()> {
    let path = config_dir().unwrap().join("omenrgb");
    if !path.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(path)? {
        println!("{}", entry?.file_name().to_string_lossy());
    }
    Ok(())
}
