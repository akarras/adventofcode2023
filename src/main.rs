use clap::{Parser, Subcommand};

use advent_utils::TestRunner;

#[derive(Parser, Debug)]
#[command()]
struct SingleTest {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    part: u8,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Single(SingleTest),
    All,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day7_part2;
pub mod day8;

fn main() {
    let Args { command } = Args::parse();
    match command {
        Commands::Single(SingleTest { day, part }) => TestRunner::run_test(day, part),
        Commands::All => TestRunner::run_all(),
    }
}
