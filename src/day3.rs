use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn item_to_priority(char: &char) -> u32 {
    match char.is_lowercase() {
        true => (*char as u32) - 96,
        false => (*char as u32) - 38,
    }
}

fn day_three_part_one() -> io::Result<u32> {
    let file = File::open("inputs/day3.txt")?;
    let reader = BufReader::new(file);

    let mut priorities = Vec::new();

    for line in reader.lines().flatten() {
        let compartments = line.split_at(line.len() / 2);
        let compartments_one_set = HashSet::<_>::from_iter(compartments.0.chars());
        let compartments_two_set = HashSet::<_>::from_iter(compartments.1.chars());

        let duplicate = compartments_one_set
            .intersection(&compartments_two_set)
            .next()
            .expect("There was no intersection between the two compartments!");

        priorities.push(item_to_priority(duplicate));
    }

    Ok(priorities.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_to_priority() {
        assert_eq!(item_to_priority(&'a'), 1);
        assert_eq!(item_to_priority(&'z'), 26);
        assert_eq!(item_to_priority(&'A'), 27);
        assert_eq!(item_to_priority(&'Z'), 52);
    }

    #[test]
    fn test_calculate_day_three_part_one() {
        let result = day_three_part_one();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 7824);
    }
}
