advent_of_code::solution!(1);

pub const DIAL_START: i64 = 50;

pub fn part_one(input: &str) -> Option<u64> {
    let mut position = DIAL_START;
    let mut zero_count = 0;
    // Split the input into lines
    let lines = input.lines();
    // For each line, split the line into a direction and a distance
    for line in lines {
        // Split the line into first character and rest
        let (direction, distance_str) = line.split_at(1);
        let distance = distance_str.parse::<i64>().unwrap();
        // If the direction is L, subtract the distance from the position
        if direction == "L" {
            position -= distance;
        } else {
            position += distance;
        }
        while position < 0 {
            position += 100;
        }
        position = position % 100;
        if position == 0 {
            zero_count += 1;
        }
    }
    return Some(zero_count);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut position = DIAL_START;
    let mut zero_count = 0;
    // Split the input into lines
    let lines = input.lines();
    // For each line, split the line into a direction and a distance
    for line in lines {
        // Split the line into first character and rest
        let (direction, distance_str) = line.split_at(1);
        let distance = distance_str.parse::<i64>().unwrap();
        // If the direction is L, subtract the distance from the position
        if direction == "L" {
            if position == 0 && distance > 0 {
                zero_count -= 1;
            }
            position -= distance;
        } else {
            position += distance;
        }

        while position < 0 {
            zero_count += 1;
            position += 100;
        }
        if position == 0 {
            zero_count += 1;
        }
        while position > 99 {
            zero_count += 1;
            position -= 100;
        }
    }
    return Some(zero_count);
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
