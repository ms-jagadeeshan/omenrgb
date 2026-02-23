use clap::{Parser, Subcommand};
use clap::builder::PossibleValuesParser;

use crate::constants::MODES;

#[derive(Parser)]
#[command(name = "kbdctl")]
#[command(version)]
#[command(about = "Control HP Omen 4 zone RGB keyboard")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Set {
        #[arg(
            help = "Target zone (0-3, zone0, zone00) or 'all'",
            value_name = "TARGET"
        )]
        target: String,

        #[arg(
            help = "Color (HEX, #HEX, shorthand, or named color)",
            value_name = "COLOR"
        )]
        value: String,
    },
    Brightness {
        #[arg(
            help="Set brightness (0-100) or +/- delta",
            value_name="VALUE"
        )]
        value: String,
    },
  Mode {
        #[arg(
            help = "Animation mode",
            value_parser = PossibleValuesParser::new(MODES),
            value_name = "MODE"
        )]
        mode: String,
    },

    Speed {
        #[arg(
            help = "Animation speed (1-10) or +/- delta",
            value_name = "VALUE"
        )]
        value: String,
    },
    Status {
        #[arg(long)]
        json: bool,
    },
    Preset {
        #[command(subcommand)]
        action: PresetCmd,
    },

    Gradient {
        #[arg(help = "Start color")]
        from: String,

        #[arg(help = "End color")]
        to: String,
    },

    Random {
        #[arg(long, help = "Randomize zone colors")]
        zones: bool,

        #[arg(long, help = "Randomize animation mode")]
        mode: bool,
    },
}

#[derive(Subcommand)]
pub enum PresetCmd {
    Save{name: String},
    Load{name: String},
    List,
}
