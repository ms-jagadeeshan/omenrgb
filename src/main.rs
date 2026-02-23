mod animation;
mod cli;
mod color;
mod constants;
mod driver;
mod presets;
mod status;
mod utils;
mod zone;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Set { target, value } => {
            let color = color::normalize_color(&value)?;

            if target.eq_ignore_ascii_case("all") {
                driver::set_all(&color)?;
            } else {
                let zone = zone::parse_zone(&target)?;
                driver::set_zone(zone, &color)?;
            }
        }
        Commands::All { color } => {
            let color = color::normalize_color(&color)?;
            driver::set_all(&color)?;
        }

        Commands::Brightness { value } => {
            let brightness = utils::parse_percentage(value, driver::get_brightness()?)?;
            driver::set_brightness(brightness)?;
        }
        Commands::Animation { mode, speed } => {
            animation::validate_mode(&mode)?;
            driver::set_mode(&mode)?;
            if !speed.eq_ignore_ascii_case("0") {
                let speed = utils::parse_range(speed, driver::get_speed()?, 0, 100)?;
                driver::set_speed(speed)?;
            }
        }
        Commands::Status { json } => {
            let status = status::read_status()?;
            if json {
                println!("{}", serde_json::to_string_pretty(&status)?);
            } else {
                println!("{}", status);
            }
        }
        Commands::Preset { action } => {
            presets::handle_preset(action)?;
        }
        Commands::Gradient { from, to } => {
            utils::apply_gradient(&from, &to)?;
        }
        Commands::Random { zones, mode } => {
            utils::apply_random(zones, mode)?;
        }
    }
    Ok(())
}
