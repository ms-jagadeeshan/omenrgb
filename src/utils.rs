use anyhow::Result;
use rand::Rng;
use crate::{driver, color};

pub fn parse_percentage(input: String, current: u8) -> Result<u8> {
    if input.starts_with('+') || input.starts_with('-') {
        let delta: i8 = input.parse()?;
        let new = (current as i16 + delta as i16).clamp(0, 100);
        return Ok(new as u8);
    }
    Ok(input.parse()?)
}

pub fn parse_range(input: String, current: u8, min: u8, max: u8) -> Result<u8> {
    if input.starts_with('+') || input.starts_with('-') {
        let delta: i8 = input.parse()?;
        let new = (current as i16 + delta as i16).clamp(min as i16, max as i16);
        return Ok(new as u8);
    }
    Ok(input.parse()?)
}

pub fn apply_gradient(from: &str, to: &str) -> Result<()> {
    let c1 = hex_to_rgb(&color::normalize_color(from)?)?;
    let c2 = hex_to_rgb(&color::normalize_color(to)?)?;

    for i in 0..4 {
        let t = i as f32 / 3.0;
        let r = lerp(c1.0, c2.0, t);
        let g = lerp(c1.1, c2.1, t);
        let b = lerp(c1.2, c2.2, t);

        driver::set_zone(i, &format!("{:02X}{:02X}{:02X}", r, g, b))?;
    }
    Ok(())
}

pub fn apply_random(zones: bool, mode: bool) -> Result<()> {
    let mut rng = rand::thread_rng();

    if zones {
        for i in 0..4 {
            let color = format!("{:06X}", rng.r#gen::<u32>() & 0xFFFFFF);
            driver::set_zone(i, &color)?;
        }
    }

    if mode {
        let modes = [
            "static","breathing","rainbow","wave","pulse",
            "chase","sparkle","candle","aurora","disco"
        ];
        driver::set_mode(modes[rng.gen_range(0..modes.len())])?;
    }

    Ok(())
}

fn hex_to_rgb(hex: &str) -> Result<(u8,u8,u8)> {
    Ok((
        u8::from_str_radix(&hex[0..2],16)?,
        u8::from_str_radix(&hex[2..4],16)?,
        u8::from_str_radix(&hex[4..6],16)?,
    ))
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}
