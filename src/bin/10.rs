use std::collections::HashMap;

use z3::ast::Int;

advent_of_code::solution!(10);

#[derive(Debug)]
pub struct Machine {
    pub target: usize,
    pub buttons: Vec<Vec<usize>>,
    pub button_masks: Vec<usize>,
    pub joltages: Vec<usize>,
}

pub const MAX_MOVES: u64 = 100000;

pub fn input_to_machines(input: &str) -> Vec<Machine> {
    input.lines().map(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let target = parts[0][1..parts[0].len() - 1].chars().enumerate().fold(0, |acc, (i, c)|
            if c == '#' { acc | (1 << i) } else { acc }
        );
        let buttons: Vec<Vec<usize>> = parts[1..parts.len() - 1].iter().map(|&p|
            p[1..p.len() - 1].split(',')
                .map(|b| b.parse::<usize>().unwrap_or(0)).collect()
            ).collect();
        let button_masks = buttons.iter().map(|b| b.iter().fold(0, |acc, b| acc | (1 << b))).collect();
        let j = parts[parts.len() - 1];
        let joltages = j[1..j.len() - 1].split(',').map(|s| s.parse::<usize>().unwrap_or(0)).collect();
        Machine { target, buttons, button_masks, joltages }
    }).collect()
}

pub fn min_moves(machine: &Machine, state: usize, depth: usize, max_depth: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if depth > max_depth || memo.get(&state).unwrap_or(&max_depth) <= &depth {
        return max_depth + 1;
    }
    memo.insert(state, depth);
    if state == machine.target {
        return depth;
    }
    machine.button_masks.iter().map(|button_mask| {
        min_moves(machine, state ^ button_mask, depth + 1, max_depth, memo)
    }).min().unwrap_or(max_depth + 1)
}

pub fn min_moves_joltage_z3(machine: &Machine) -> u64 {
    // Solve the minimum number of button presses to reach the target joltages
    // Machine has a set of buttons that increment the joltage in given slots by 1 each time they are pressed
    // The machine has a target joltages array that must be reached
    let optimizer = z3::Optimize::new();
    // Button presses
    let button_presses = (0..machine.buttons.len()).map(|i|
        Int::fresh_const(&format!("button_press_{}", i))).collect::<Vec<_>>();
    // Derive joltages from button presses
    let joltages = (0..machine.joltages.len()).map(|slot| {
        let buttons_for_slot = machine.buttons.iter().enumerate()
            .filter(|(_, buttons)| buttons.contains(&slot)).map(|(i, _)| &button_presses[i]).collect::<Vec<_>>();
        Int::add(&buttons_for_slot)
    }).collect::<Vec<_>>();
    // Constrain joltages to equal target joltages
    for i in 0..machine.joltages.len() {
        optimizer.assert(&joltages[i].eq(machine.joltages[i] as u32));
    }
    // Constrain button presses to be non-negative
    for button_press in button_presses.iter() {
        optimizer.assert(&button_press.ge(0));
    }
    // Minimize the sum of button presses
    let sum_of_button_presses = Int::add(&button_presses.iter().collect::<Vec<_>>());
    optimizer.minimize(&sum_of_button_presses);
    if let z3::SatResult::Sat = optimizer.check(&[]) {
        let model = optimizer.get_model().unwrap();
        let moves = model.eval(&sum_of_button_presses, true).unwrap();
        moves.as_u64().unwrap_or(MAX_MOVES + 1)
        // let moves = button_presses.iter().map(|b| model.eval(b, true).unwrap().as_u64().unwrap_or(MAX_MOVES + 1)).sum();
        // moves
    } else {
        MAX_MOVES + 1
    }
}


pub fn part_one(input: &str) -> Option<u64> {
    Some(
            input_to_machines(input).iter().map(|machine| {
                let moves = min_moves(machine, 0, 0, 20, &mut HashMap::new());
                moves as u64
        }).sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input_to_machines(input).iter().map(|machine| {
            // println!("machine: {:?}", machine);
            let moves = min_moves_joltage_z3(machine);
            // println!("moves: {}", moves);
            moves as u64
        }).sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
