pub mod day01;
pub mod day02;
pub mod day03;

use aoc::Context;

pub fn get_day_solver(day: u8) -> fn(&mut Context) -> () {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        _ => unimplemented!(),
    }
}
