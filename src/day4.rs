use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Range,
};

fn parse_section_pattern(pattern: &str) -> Range<u32> {
    let (lower, upper) = pattern.split_once('-').unwrap();
    Range {
        start: lower.parse::<u32>().unwrap(),
        end: upper.parse::<u32>().unwrap(),
    }
}

fn parse_line(line: &str) -> (Range<u32>, Range<u32>) {
    let (left_pattern, right_pattern) = line.split_once(',').unwrap();
    (
        parse_section_pattern(left_pattern),
        parse_section_pattern(right_pattern),
    )
}

fn range_fully_contains_other<T: PartialOrd>(left: &Range<T>, right: &Range<T>) -> bool {
    left.start <= right.start && left.end >= right.end
}

fn ranges_fully_overlap<T: PartialOrd>(left: &Range<T>, right: &Range<T>) -> bool {
    range_fully_contains_other(left, right) || range_fully_contains_other(right, left)
}

fn range_partially_contains_other<T: PartialOrd>(left: &Range<T>, right: &Range<T>) -> bool {
    (left.start <= right.start && left.end >= right.start)
        || (left.start >= right.start && left.end <= right.end)
}

fn ranges_partially_overlap<T: PartialOrd>(left: &Range<T>, right: &Range<T>) -> bool {
    range_partially_contains_other(left, right) || range_partially_contains_other(right, left)
}

fn day_four_part_one() -> io::Result<u32> {
    let file = File::open("inputs/day4.txt")?;
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .flatten()
        .map(|line| parse_line(&line))
        .map(|ranges| ranges_fully_overlap(&ranges.0, &ranges.1) as u32)
        .sum::<u32>();

    Ok(result)
}

fn day_four_part_two() -> io::Result<u32> {
    let file = File::open("inputs/day4.txt")?;
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .flatten()
        .map(|line| parse_line(&line))
        .map(|ranges| ranges_partially_overlap(&ranges.0, &ranges.1) as u32)
        .sum::<u32>();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_fully_contains_other() {
        assert!(range_fully_contains_other(
            &Range { start: 2, end: 8 },
            &Range { start: 3, end: 7 }
        ));

        assert!(range_fully_contains_other(
            &Range { start: 4, end: 6 },
            &Range { start: 6, end: 6 }
        ));

        assert!(!range_fully_contains_other(
            &Range { start: 6, end: 6 },
            &Range { start: 4, end: 6 },
        ));
    }

    #[test]
    fn test_range_partially_contains_other() {
        assert!(range_partially_contains_other(
            &Range { start: 5, end: 7 },
            &Range { start: 7, end: 9 }
        ));

        assert!(range_partially_contains_other(
            &Range { start: 2, end: 8 },
            &Range { start: 3, end: 7 }
        ));

        assert!(range_partially_contains_other(
            &Range { start: 6, end: 6 },
            &Range { start: 4, end: 6 }
        ));

        assert!(range_partially_contains_other(
            &Range { start: 2, end: 6 },
            &Range { start: 4, end: 8 }
        ));
    }

    #[test]
    fn test_day_four_part_one() {
        let result = day_four_part_one();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 441)
    }

    #[test]
    fn test_day_four_part_two() {
        let result = day_four_part_two();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 861)
    }
}
