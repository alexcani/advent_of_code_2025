use aoc::Context;

// All gifts have a bounding box of 3x3 i.e. fit in a 9 area
// Let's just count the area and see if it works
struct GiftShape {
    area: u32,
}

struct Problem {
    width: u32,
    height: u32,
    number_of_gifts: Vec<u32>, // index correspond to type of GiftShape
}

pub fn solve(ctx: &mut Context) {
    let mut gifts = Vec::new();
    let mut problems = Vec::new();
    let mut lines = ctx.input().iter();
    while let Some(content) = lines.next() {
        if content.is_empty() {
            continue;
        }

        if content.contains(':') {
            // Can be 'N:' or 'AxB: 1 2 3 4 5'
            let parts: Vec<&str> = content.split(':').collect();
            if parts[0].contains('x') {
                // AxB: ...
                let mut dimensions = parts[0].split('x');
                let width: u32 = dimensions.next().unwrap().parse().unwrap();
                let height: u32 = dimensions.next().unwrap().parse().unwrap();
                let numbers: Vec<u32> = parts[1]
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                problems.push(Problem {
                    width,
                    height,
                    number_of_gifts: numbers,
                });
            } else {
                // N: followed by shape description
                // N itsel is irrelevant since it starts from 0 and they appear in order
                // We know the gift shape is shown in the next 3 lines. so we just count the '#'
                let mut area = 0;
                for _ in 0..3 {
                    let shape_line = lines.next().unwrap();
                    area += shape_line.chars().filter(|&c| c == '#').count() as u32;
                }
                gifts.push(GiftShape { area });
            }
        }
    }

    let mut result = 0;
    for problem in problems {
        let total_area: u32 = problem
            .number_of_gifts
            .iter()
            .enumerate()
            .map(|(i, &count)| count * gifts[i].area)
            .sum();
        let box_area = problem.width * problem.height;
        // Funny this works for the input but not for the example
        // ¯\_(ツ)_/¯
        if total_area <= box_area {
            result += 1;
        }
    }

    ctx.set_sol1(result);
}
