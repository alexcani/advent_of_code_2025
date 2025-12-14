use std::collections::HashMap;

use aoc::Context;

#[derive(Default)]
struct Operation {
    operands: Vec<u64>,
    operator: char,
}

pub fn solve(ctx: &mut Context) {
    let mut operations = HashMap::new();
    for line in ctx.input() {
        line.split_whitespace()
            .enumerate()
            .for_each(|(index, data)| {
                let operation = operations.entry(index).or_insert(Operation {
                    operands: Vec::new(),
                    operator: ' ',
                });
                if let Ok(value) = data.parse::<u64>() {
                    operation.operands.push(value);
                } else if data == "+" || data == "*" {
                    operation.operator = data.chars().next().unwrap();
                }
            });
    }

    let mut result1: u64 = 0;
    for op in operations.values() {
        let res = match op.operator {
            '+' => op.operands.iter().sum(),
            '*' => op.operands.iter().product(),
            _ => 0,
        };
        result1 += res;
    }
    ctx.set_sol1(result1);

    let mut operations = vec![Operation::default()];
    let max_cols = ctx.input().iter().map(|l| l.len()).max().unwrap();
    let last_digit_line_index = ctx.input().len() - 2;
    let mut operation_n = 0;
    for col in 0..max_cols {
        let mut operand = "".to_string();
        for line in ctx.input().iter().take(last_digit_line_index + 1) {
            operand.push(line.chars().nth(col).unwrap_or(' '));
        }
        if operand.trim().is_empty() {
            operation_n += 1;
            operations.push(Operation::default());
            continue;
        }
        let operation = &mut operations[operation_n];
        operation
            .operands
            .push(operand.trim().parse::<u64>().unwrap());
    }

    ctx.input()
        .last()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .for_each(|(index, data)| {
            let operation = &mut operations[index];
            if data == "+" || data == "*" {
                operation.operator = data.chars().next().unwrap();
            } else {
                panic!("Unexpected data in last line: {}", data);
            }
        });

    let mut result2: u64 = 0;
    for op in &operations {
        let res = match op.operator {
            '+' => op.operands.iter().sum(),
            '*' => op.operands.iter().product(),
            _ => 0,
        };
        result2 += res;
    }
    ctx.set_sol2(result2);
}
