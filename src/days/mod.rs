pub mod day01;

use aoc::Context;

pub fn get_day_solver(day: u8) -> fn(&mut Context) -> () {
    match day {
        1 => day01::solve,
        _ => unimplemented!(),
    }
}
