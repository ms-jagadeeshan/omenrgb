use anyhow::{anyhow,Result};

pub fn normalize_color(input:&str)->Result<String>{

    let mut s = input.trim().to_lowercase();

    let named= match s.as_str(){
        "red" => "ff0000",
        "green" => "00ff00",
        "blue" => "0000ff",
        "yellow" => "ffff00",
        "cyan" => "00ffff",
        "magenta" => "ff00ff",
        "white" => "ffffff",
        "black" => "000000",
        "gray" => "808080",
        "orange" => "ff4500",
        "purple" => "8000ff",
        "pink" => "ffc0cb",
        "lime" => "00ff00",
        "teal" => "008080",
        "maroon" => "800000",
        "navy" => "000080",
        "aqua" => "00ffff",
        "olive" => "808000",
        "silver" => "c0c0c0",
        "brown" => "a52a2a",
        "lavender" => "e6e6fa",
        _=> &s,
    };

    s=named.to_string();

    if s.starts_with('#'){
        s=s[1..].to_string();
    }

    if s.len()==3{
        s=s.chars().map(|c| format!("{c}{c}")).collect();
    }

    if s.len()!=6 || !s.chars().all(|c| c.is_ascii_hexdigit()){
        return Err(anyhow!("Invalid color format"));
    }
    Ok(s.to_uppercase())
}


