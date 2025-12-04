use aoc::Context;

struct Dial {
    position: i32,
    times_at_zero: i32,
}

impl Dial {
    const MAX_VALUE: i32 = 99;
    pub fn new(initial_position: i32) -> Self {
        Dial {
            position: initial_position,
            times_at_zero: 0,
        }
    }

    pub fn turn_right(&mut self, steps: i32) {
        // Calculate how many times we would wrap around
        let will_wrap = steps / (Self::MAX_VALUE + 1);
        let remainder_steps = steps % (Self::MAX_VALUE + 1);
        self.times_at_zero += will_wrap;

        // Apply only the remainder steps
        self.position += remainder_steps;
        if self.position > Self::MAX_VALUE {
            self.times_at_zero += 1;
            self.position %= Self::MAX_VALUE + 1;
        }
    }

    pub fn turn_left(&mut self, steps: i32) {
        let initial_position = self.position;
        // Calculate how many times we would wrap around
        let will_wrap = steps / (Self::MAX_VALUE + 1);
        let remainder_steps = steps % (Self::MAX_VALUE + 1);
        self.times_at_zero += will_wrap;

        // Apply only the remainder steps
        self.position -= remainder_steps;
        if self.position < 0 {
            if initial_position != 0 {
                self.times_at_zero += 1;
            }
            self.position += Self::MAX_VALUE + 1;
        }

        // When turning left, we can also land exactly on zero without wrapping
        if self.position == 0 {
            self.times_at_zero += 1;
        }
    }

    pub fn current_position(&self) -> i32 {
        self.position
    }
}

pub fn solve(ctx: &mut Context) {
    let mut dial = Dial::new(50);
    let mut result1 = 0;
    for line in ctx.input() {
        // first character indicates direction, rest is number of steps
        let direction = line.chars().next().unwrap();
        let steps: i32 = line[1..].parse().unwrap();
        match direction {
            'R' => dial.turn_right(steps),
            'L' => dial.turn_left(steps),
            _ => panic!("Invalid direction"),
        }
        if dial.current_position() == 0 {
            result1 += 1;
        }
        if dial.current_position() < 0 || dial.current_position() > Dial::MAX_VALUE {
            panic!("Dial position out of bounds");
        }
    }
    ctx.set_sol1(result1);
    ctx.set_sol2(dial.times_at_zero);
}
