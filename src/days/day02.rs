use aoc::Context;

use std::collections::HashSet;

pub fn solve_range(range: &(&str, &str)) -> (u64, u64) {
    let (start, end) = range;
    let n_digits_end = end.len();
    let start_num: u64 = start.parse().unwrap();
    let end_num: u64 = end.parse().unwrap();

    let digits = (0..=9).map(|d| d.to_string()).collect::<Vec<String>>();

    // Set containing the numbers to explore, i.e. either adding a new digit
    // or repeating the current sequence and moving it to the duplicated_set
    let mut queue: HashSet<String> = (0..=9).map(|d| d.to_string()).collect();

    let mut solution_set = HashSet::new();
    let mut solution_second_set = HashSet::new();

    // We need to construct numbers with a sequence of digits repeated
    // twice (and only twice)
    // e.g. 1111, 14411441
    // and check if they are within the range

    while let Some(current) = queue.iter().next().cloned() {
        queue.remove(&current);
        let current_len = current.len();
        let leading_zero = current.starts_with('0');

        // Vector of numbers resulting by repeating the sequence until
        // we exceed the maximum length
        let mut repeated_numbers = Vec::new();
        let mut repeated = format!("{}{}", current, current);
        while repeated.len() <= n_digits_end {
            repeated_numbers.push(repeated.clone());
            repeated = format!("{}{}", current, repeated);
        }

        let matches = |num_str: &str| {
            let num: u64 = num_str.parse().unwrap();
            num >= start_num && num <= end_num && !leading_zero
        };

        if !repeated_numbers.is_empty() {
            // First number is the one with two repetitions, special
            // case for the first solution
            let mut iter = repeated_numbers.into_iter();
            let first = iter.next().unwrap();
            if matches(&first) {
                let num: u64 = first.parse().unwrap();
                solution_set.insert(num);
                solution_second_set.insert(num); // also for the second solution
            }
            for num_str in iter {
                if matches(&num_str) {
                    let num: u64 = num_str.parse().unwrap();
                    solution_second_set.insert(num);
                }
            }
        }

        if current_len < n_digits_end / 2 {
            // We can still add more digits to the front
            for d in &digits {
                queue.insert(format!("{}{}", d, current));
            }
        }
    }

    (solution_set.iter().sum(), solution_second_set.iter().sum())
}

pub fn solve(ctx: &mut Context) {
    let ranges: Vec<(&str, &str)> = ctx.input()[0]
        .split(',')
        .map(|x| {
            let mut range = x.split('-');
            let start = range.next().unwrap();
            let end = range.next().unwrap();
            (start, end)
        })
        .collect();

    let (result1, result2) = ranges
        .iter()
        .map(solve_range)
        .reduce(|(acc1, acc2), (x1, x2)| (acc1 + x1, acc2 + x2))
        .unwrap();
    ctx.set_sol1(result1);
    ctx.set_sol2(result2);
}
