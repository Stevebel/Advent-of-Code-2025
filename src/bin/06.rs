advent_of_code::solution!(6);

pub struct Problem {
    numbers: Vec<u64>,
    operator: char,
}

pub fn parse_problems(input: &str) -> Vec<Problem> {
    // Each line contains the numbers for each problem, separated by spaces
    // the last line contains the operator
    let mut lines = input.lines().collect::<Vec<&str>>();
    let operators = lines.pop().unwrap_or_default().split_whitespace().collect::<Vec<&str>>();
    let mut problems: Vec<Problem> = operators.iter().map(|&operator| {
        Problem { numbers: Vec::<u64>::new(), operator: operator.chars().next().unwrap() }
    }).collect();
    for line in lines {
        line.split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .enumerate()
            .for_each(|(index, number)| {
                problems[index].numbers.push(number);
            });
    }
    problems
}

pub fn parse_problems2(input: &str) -> Vec<Problem> {
    // Numbers are vertical columns right to left, operators still in last line
    // Pivot the columns into rows, put the last character of each line into the first pivoted_line and so on
    let pivoted = input.lines().map(|line| line.chars().rev())
        .fold(Vec::<String>::new(), |mut pivoted_lines, line| {
            line.enumerate().for_each(|(index, c)| {
                if index >= pivoted_lines.len() {
                    pivoted_lines.push(String::new());
                }
                if !c.is_whitespace() {
                    pivoted_lines[index].push(c);
                }
            });
            pivoted_lines
        }).into_iter().filter(|line| !line.is_empty()).collect::<Vec<String>>();
    let mut problems: Vec<Problem> = Vec::new();
    let mut current_problem = Problem { numbers: Vec::new(), operator: '+' };
    for line in pivoted {
        // Check for operator at the end of the line
        if line.ends_with('*') || line.ends_with('+') {
            current_problem.numbers.push(line[..line.len() - 1].parse::<u64>().unwrap_or(0));
            current_problem.operator = line.chars().last().unwrap();
            problems.push(current_problem);
            current_problem = Problem { numbers: Vec::new(), operator: '+' };
        } else {
            current_problem.numbers.push(line.parse::<u64>().unwrap_or(0));
        }
    }
    problems
}

pub fn solve_problem(problem: &Problem) -> u64 {
    match problem.operator {
        '*' => problem.numbers.iter().product(),
        '+' => problem.numbers.iter().sum(),
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse_problems(input).iter().map(solve_problem).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse_problems2(input).iter().map(solve_problem).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
