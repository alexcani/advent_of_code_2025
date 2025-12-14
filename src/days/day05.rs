use aoc::Context;

pub fn solve(ctx: &mut Context) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    for line in ctx.input() {
        if line.is_empty() {
            continue;
        }

        let mut split = line.split('-');
        let first_num = split.next().unwrap().parse::<u64>().unwrap();
        if let Some(second_str) = split.next() {
            let second_num = second_str.parse::<u64>().unwrap();
            ranges.push((first_num, second_num));
        } else {
            ids.push(first_num);
        }
    }

    let mut result = 0;
    for id in &ids {
        for (start, end) in &ranges {
            if id >= start && id <= end {
                result += 1;
                break;
            }
        }
    }

    ctx.set_sol1(result);

    // Sort ascending by start value
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged_ranges = Vec::new();
    let mut current_range = ranges[0];
    ranges.iter().skip(1).for_each(|range| {
        if range.0 <= current_range.1 {
            current_range = (current_range.0, current_range.1.max(range.1));
        } else {
            // No overlap, push the current range and move to the next
            merged_ranges.push(current_range);
            current_range = *range;
        }
    });
    merged_ranges.push(current_range);

    let mut result2 = 0;
    for (start, end) in &merged_ranges {
        result2 += end - start + 1;
    }

    ctx.set_sol2(result2);
}
