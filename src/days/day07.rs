use aoc::Context;
use aoc::Grid;
use aoc::Point;

use std::collections::HashMap;
use std::collections::VecDeque;

pub fn solve1(ctx: &mut Context) {
    let mut map = Grid::parse(ctx.input());
    let start = map.find(b'S').unwrap();

    let mut queue = VecDeque::new();
    queue.push_front(start);

    let mut num_splits = 0;

    while let Some(pos) = queue.pop_back() {
        if !map.contains(pos) {
            continue;
        }

        match map[pos] {
            b'S' => {
                queue.push_front(pos.down(1));
            }
            b'.' => {
                map[pos] = b'|';
                queue.push_front(pos.down(1));
            }
            b'^' => {
                num_splits += 1;
                queue.push_front(pos.left(1));
                queue.push_front(pos.right(1));
            }
            b'|' => {}
            _ => unreachable!(),
        }
    }

    ctx.set_sol1(num_splits);
}

pub fn solve2_recursion(map: &Grid<u8>, start: &Point, cache: &mut HashMap<Point, u64>) -> u64 {
    if !map.contains(*start) {
        // reached the bottom
        return 1;
    }

    if let Some(cached) = cache.get(start) {
        return *cached;
    }

    let result = match map[*start] {
        b'S' => solve2_recursion(map, &start.down(1), cache),
        b'.' => solve2_recursion(map, &start.down(1), cache),
        b'^' => {
            let left_paths = solve2_recursion(map, &start.left(1), cache);
            let right_paths = solve2_recursion(map, &start.right(1), cache);
            left_paths + right_paths
        }
        _ => 0,
    };

    cache.insert(*start, result);
    result
}

pub fn solve2(ctx: &mut Context) {
    let map = Grid::parse(ctx.input());
    let start = map.find(b'S').unwrap();
    let mut cache = HashMap::new();
    let num_paths = solve2_recursion(&map, &start, &mut cache);
    ctx.set_sol2(num_paths);
}

pub fn solve(ctx: &mut Context) {
    solve1(ctx);
    solve2(ctx);
}
