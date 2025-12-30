use good_lp::{Expression, Solution, SolverModel, Variable, microlp, variable, variables};
use std::collections::{HashSet, VecDeque};

use aoc::Context;

#[derive(Clone, Debug)]
struct Button {
    connections: Vec<usize>, // indices of connected indicator lights
}

struct Problem {
    buttons: Vec<Button>,
    indicator_lights: Vec<bool>,
    goal_states: Vec<bool>,
    joltages_goals: Vec<u64>,
}

pub fn solve(ctx: &mut Context) {
    // One problem per line
    let mut problems = Vec::new();
    ctx.input().iter().for_each(|line| {
        let parts: Vec<&str> = line.split(' ').collect();
        let goal_states: Vec<bool> = parts[0]
            .trim_start_matches('[')
            .trim_end_matches(']')
            .chars()
            .map(|c| c == '#')
            .collect();
        let mut buttons = Vec::new();
        for part in &parts[1..(parts.len() - 1)] {
            let conn_str = part.trim_start_matches('(').trim_end_matches(')');
            let connections = conn_str
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            buttons.push(Button { connections });
        }
        let joltage_str = parts[parts.len() - 1]
            .trim_start_matches('{')
            .trim_end_matches('}');
        let joltages_goals: Vec<u64> = joltage_str
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        assert!(goal_states.len() == joltages_goals.len());
        problems.push(Problem {
            buttons,
            indicator_lights: vec![false; goal_states.len()],
            goal_states,
            joltages_goals,
        });
    });

    let result1: usize = problems.iter().map(solve_problem1).sum();
    ctx.set_sol1(result1);

    let result2: usize = problems.iter().map(solve_problem2).sum();
    ctx.set_sol2(result2);
}

fn solve_problem1(problem: &Problem) -> usize {
    // BFS over button presses to find the minimum press sequence to reach the goal state
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((problem.indicator_lights.clone(), Vec::new()));
    while let Some((current_state, press_sequence)) = queue.pop_front() {
        if current_state == problem.goal_states {
            return press_sequence.len();
        }
        for (button_idx, button) in problem.buttons.iter().enumerate() {
            // Apply button press
            let mut next_state = current_state.clone();
            for &light_idx in &button.connections {
                next_state[light_idx] = !next_state[light_idx];
            }

            // Because we're doing BFS, we don't have to worry about the sequence length here,
            // since the first time we reach a state is guaranteed to be the shortest path to that state.
            if visited.insert(next_state.clone()) {
                let mut next_sequence = press_sequence.clone();
                next_sequence.push(button_idx);
                queue.push_back((next_state, next_sequence));
            }
        }
    }

    unreachable!()
}

fn solve_problem2(problem: &Problem) -> usize {
    // Problem 2 is a linear optimization problem. We need to find the minimum number of button presses
    // to reach the goal joltages, given that each button press increases the joltages in the same index by 1.
    // Suppose we have buttons A, B, C, D, E, F. And each button connects to the following joltage indicators:
    // A -> [3]
    // B -> [1, 3]
    // C -> [2]
    // D -> [2, 3]
    // E -> [0, 2]
    // F -> [0, 1]
    // And the joltage vector is represented as [j0, j1, j2, j3].
    // Let 'a' be the number of times button A is pressed, 'b' for button B, etc.
    // Then we have the following equations for each joltage
    // j0 = e + f
    // j1 = b + f
    // j2 = c + d + e
    // j3 = a + b + d
    // And let the goal joltages be g0, g1, g2, g3, represented as [g0, g1, g2, g3].
    // We need to solve the following system of equations:
    // e + f = g0
    // b + f = g1
    // c + d + e = g2
    // a + b + d = g3
    // While minimizing the total number of button presses: a + b + c + d + e + f
    // This can be solved using integer linear programming.
    // Matrix form:
    // [0 0 0 0 1 1] [a]   [g0]
    // [0 1 0 0 0 1] [b] = [g1]
    // [0 0 1 1 1 0] [c]   [g2]
    // [1 1 0 1 0 0] [d]   [g3]
    //               [e]
    //               [f]
    // Objective: Minimize [1 1 1 1 1 1] [a b c d e f]^T

    let mut model = variables!();
    // One integer variable per button, representing the number of times it is pressed
    let vars = vec![variable().min(0).integer(); problem.buttons.len()];
    let vars: Vec<Variable> = model.add_all(vars);
    let objective: Expression = vars.iter().sum();
    let mut solver = model.minimise(&objective).using(microlp);

    // For each joltage indicator, find out which buttons connect to it and create a constraint
    problem
        .joltages_goals
        .iter()
        .enumerate()
        .for_each(|(joltage_idx, &goal)| {
            let connected_buttons: Vec<usize> = problem
                .buttons
                .iter()
                .enumerate()
                .filter_map(|(button_idx, button)| {
                    if button.connections.contains(&joltage_idx) {
                        Some(button_idx)
                    } else {
                        None
                    }
                })
                .collect();
            let expr: Expression = connected_buttons.iter().map(|&idx| vars[idx]).sum();
            solver.add_constraint(expr.eq(goal as f64));
        });
    let solution = solver.solve().unwrap();
    solution.eval(&objective).round() as usize
}
