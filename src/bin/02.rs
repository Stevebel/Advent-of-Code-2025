advent_of_code::solution!(2);

pub fn is_invalid(num: u64) -> bool {
    // The ID is invalid if it consists of a number repeated twice
    let num_str = num.to_string();
    // Can't be repeated twice if the length is odd
    if num_str.len() % 2 != 0 {
        return false;
    }
    // Split the number into two halves
    let (first_half, second_half) = num_str.split_at(num_str.len() / 2);
    return first_half == second_half;
}

pub fn is_invalid_any(num: u64) -> bool {
    // The ID is invalid if it consists of a number repeated any number of times
    let num_str = num.to_string();
    let length = num_str.len();
    for n in 1..=(length / 2) {
        // Can't be repeated every n characters if not divisible by n
        if length % n != 0 {
            continue;
        }
        let mut repeated = true;
        let mut offset = n;
        let first_segment = &num_str[0..n];
        while repeated && (offset + n) <= length  {
            let segment = &num_str[offset..offset+n];
            // println!("Num: {}, N: {}, Start: {}, Segment: {}", num, n, first_segment, segment);
            repeated = repeated && segment == first_segment;
            offset += n;
        }
        if repeated {
            return true;
        }
    }
    return false;
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_sum = 0;
    // Split on commas
    let id_ranges = input.split(',');
    for id_range in id_ranges {
        let (start, end) = id_range.split_once('-').unwrap();
        let start = start.trim().parse::<u64>().unwrap();
        let end = end.trim().parse::<u64>().unwrap();
        for id in start..=end {
            if is_invalid(id) {
                invalid_sum += id;
            }
        }
    }
    return Some(invalid_sum);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut invalid_sum = 0;
    // Split on commas
    let id_ranges = input.split(',');
    for id_range in id_ranges {
        let (start, end) = id_range.split_once('-').unwrap();
        let start = start.trim().parse::<u64>().unwrap();
        let end = end.trim().parse::<u64>().unwrap();
        for id in start..=end {
            if is_invalid_any(id) {
                invalid_sum += id;
            }
        }
    }
    return Some(invalid_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        assert!(is_invalid_any(11));
        assert!(is_invalid_any(111111111));
        assert!(is_invalid_any(1212));
        assert!(is_invalid_any(121212));
        assert!(is_invalid_any(12345671234567));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
