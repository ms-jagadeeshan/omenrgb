mod animation;
mod cli;
mod color;
mod driver;
mod presets;
mod status;
mod utils;
mod zone;
mod constants;

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

        Commands::Brightness { value } => {
            let brightness = utils::parse_percentage(value, driver::get_brightness()?)?;
            driver::set_brightness(brightness)?;
        }
        Commands::Mode { mode } => {
            animation::validate_mode(&mode)?;
            driver::set_mode(&mode)?;
        }
        Commands::Speed { value } => {
            let speed = utils::parse_range(value, driver::get_speed()?, 0, 100)?;
            driver::set_speed(speed)?;
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
