mod day11;

use anyhow::Result;
use std::process;
use std::str::FromStr;

pub enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            _ => Err(()),
        }
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    let part = args
        .nth(1)
        .unwrap_or_else(|| show_help_and_exit())
        .parse()
        .unwrap_or_else(|_| show_help_and_exit());

    day11::solve(part)?;

    Ok(())
}

fn show_help_and_exit() -> ! {
    let mut args = std::env::args();
    let cmd = args.next().expect("No command");

    eprintln!("{cmd} <part (1 or 2)>");
    process::exit(1);
}
