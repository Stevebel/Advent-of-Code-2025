advent_of_code::solution!(5);

pub fn split_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, numbers) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap_or(("0", "0"));
            (start.parse::<u64>().unwrap_or(0), end.parse::<u64>().unwrap_or(0))
        })
        .collect();
    let numbers = numbers
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    (ranges, numbers)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, numbers) = split_input(input);
    Some(
        numbers.iter().filter(|&number| {
            ranges.iter().any(|(start, end)| number >= start && number <= end)
        }).count() as u64
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = split_input(input);

    ranges.sort_by_key(|(start, _)| *start);
    let (first_start, first_end) = ranges.remove(0);
    let (_, sum) = ranges.iter().fold(
        (first_end, first_end - first_start + 1),
        |(last_end, sum), &(start, end)| {
            if start > last_end {
                // Past the end of the current range, so we add the current range to the sum
                (end, sum + (end - start) + 1)
            } else {
                // Inside the current range, so we update the end and add the difference to the sum
                let new_end = end.max(last_end);
                (new_end, sum + (new_end - last_end))
            }
        }
    );

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
