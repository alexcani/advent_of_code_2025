use aoc::Context;

use aoc::Grid;

pub fn remove_rolls(grid: &mut Grid<u8>) -> u32 {
    let mut to_remove = Vec::new();

    for (pos, value) in grid.iter() {
        if *value != b'@' {
            continue;
        }

        let mut n_of_neighboring_rolls = 0;
        for neighbor in pos.all_neighbors() {
            if let Some(&neighbor_value) = grid.get(neighbor)
                && neighbor_value == b'@'
            {
                n_of_neighboring_rolls += 1;
            }
        }

        if n_of_neighboring_rolls < 4 {
            to_remove.push(pos);
        }
    }

    let n_removed = to_remove.len() as u32;
    for pos in to_remove {
        grid[&pos] = b'.';
    }

    n_removed
}

pub fn solve(ctx: &mut Context) {
    let mut grid = Grid::parse(ctx.input());

    // First iteration is solution to problem 1
    let mut total_removed = remove_rolls(&mut grid);
    ctx.set_sol1(total_removed);

    // Repeat until we reach a stable state
    loop {
        let n_removed = remove_rolls(&mut grid);
        if n_removed == 0 {
            break;
        }
        total_removed += n_removed;
    }
    ctx.set_sol2(total_removed);
}
