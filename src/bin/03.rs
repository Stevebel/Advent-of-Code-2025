advent_of_code::solution!(3);

pub fn input_to_banks(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| line.chars().into_iter())
        .map(|c| c.map(|c| c.to_digit(10).unwrap_or(0)).collect())
        .collect()
}

pub fn find_max_in_range(vec: &Vec<u32>, start: usize, end: usize) -> (usize, u64) {
    let mut max = 0;
    let mut max_index = 0;
    for i in start..end {
        if vec[i] > max {
            max = vec[i];
            max_index = i;
        }
    }
    return (max_index, max as u64);
}

pub fn max_joltage(bank: &Vec<u32>, digits: usize) -> u64 {
    let mut joltage = 0;
    let mut start_index = 0;
    for i in 1..=digits {
        joltage *= 10;
        let (max_index, max_digit) = find_max_in_range(bank, start_index, bank.len() - (digits - i));
        joltage += max_digit;
        start_index = max_index + 1;
    }
    return joltage;
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input_to_banks(input).iter()
        .map(|bank| max_joltage(bank, 2)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input_to_banks(input).iter()
        .map(|bank| max_joltage(bank, 12)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
