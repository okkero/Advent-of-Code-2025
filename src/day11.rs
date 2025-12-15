mod input;
mod part1;
mod part2;

use crate::Part;
use anyhow::Result;

pub fn solve(part: Part) -> Result<()> {
    const INPUT: &str = include_str!("day11/input.txt");

    match part {
        Part::One => part1::solve(INPUT)?,
        Part::Two => part2::solve(INPUT)?,
    }

    Ok(())
}
