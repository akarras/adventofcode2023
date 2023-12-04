use clap::Parser;
use day1::{day_1_part_1, day_1_part_2};
use day2::{day_2_part_1, day_2_part_2};
use day3::{day_3_part_1, day_3_part_2};
use day4::{day_4_part_1, day_4_part_2};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    part: u8,
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

fn main() {
    let args = Args::parse();

    match (args.day, args.part) {
        (1, 1) => day_1_part_1(),
        (1, 2) => day_1_part_2(),
        (2, 1) => day_2_part_1(),
        (2, 2) => day_2_part_2(),
        (3, 1) => day_3_part_1(),
        (3, 2) => day_3_part_2(),
        (4, 1) => day_4_part_1(),
        (4, 2) => day_4_part_2(),
        _ => todo!(),
    }
}
