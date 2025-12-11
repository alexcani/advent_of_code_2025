use aoc::Context;

fn solve1(ctx: &Context) -> u32 {
    let mut result = 0;
    for line in ctx.input() {
        let chars = line.chars().collect::<Vec<char>>();
        let size = line.len();
        let mut largest = 0;
        let mut largest_pos = 0;
        for (i, &ch) in chars.iter().enumerate().take(size - 1) {
            let num = ch.to_digit(10).unwrap();
            if num > largest {
                largest = num;
                largest_pos = i;
            }
        }

        let mut next_largest = 0;
        for (_, &ch) in chars.iter().enumerate().skip(largest_pos + 1) {
            let num = ch.to_digit(10).unwrap();
            if num > next_largest {
                next_largest = num;
            }
        }

        result += largest * 10 + next_largest;
    }
    result
}

// picks the largest digit in numbers between min_index and max_index (inclusive),
// while minimizing the index
fn pick_largest_digit(
    numbers: &[(u64, usize)],
    min_index: usize,
    max_index: usize,
) -> (u64, usize) {
    // Since numbers is sorted descending by digit, we can just find the first
    // the smallest
    let valid: Vec<_> = numbers
        .iter()
        .filter(|(_, index)| *index >= min_index && *index <= max_index)
        .collect();

    let largest_digit = valid[0].0; // First is largest due to descending sort

    *valid
        .iter()
        .filter(|(digit, _)| *digit == largest_digit)
        .min_by_key(|(_, index)| index)
        .copied()
        .unwrap()
}

fn solve2(ctx: &Context) -> u64 {
    // in this one we need to select the 12 largest digits from each line in the order
    // they appear, without reordering
    let mut result: u64 = 0;
    for line in ctx.input() {
        let numbers = line
            .chars()
            .enumerate()
            .map(|(index, ch)| (ch.to_digit(10).unwrap() as u64, index))
            .collect::<Vec<(u64, usize)>>();

        // Sort by digit descending
        let mut sorted_numbers = numbers.clone();
        sorted_numbers.sort_by(|a, b| b.0.cmp(&a.0));

        let mut last_index = 0;
        let mut line_result = 0;
        for i in 0..12 {
            let (digit, index) =
                pick_largest_digit(&sorted_numbers, last_index, line.len() - (12 - i));
            line_result = line_result * 10 + digit;
            last_index = index + 1;
        }
        result += line_result;
    }
    result
}

pub fn solve(ctx: &mut Context) {
    ctx.set_sol1(solve1(ctx));
    ctx.set_sol2(solve2(ctx));
}
