use std::path::Path;

use crate::utils::load_file_contents;

fn concat_vec_of_int(vec: &[u32]) -> u32 {
    vec.iter().fold(0, |acc, elem| acc * 10 + elem)
}

fn calculate_elf_calorie_counts(file_contents: &str) -> Vec<u32> {
    let mut elf_idx = 0;
    let mut num_breaks = 0;
    let mut current_food_item = Vec::new();
    let mut elf_calorie_counts = vec![0];

    for char in file_contents.chars() {
        match char {
            '\n' => num_breaks += 1,
            val => {
                current_food_item.push(val.to_digit(10).unwrap());
                num_breaks = 0;
            }
        }
        match num_breaks {
            // if 1 break then we are on next food item
            1 => {
                elf_calorie_counts[elf_idx] += concat_vec_of_int(&current_food_item);
                current_food_item.clear();
            }
            // if 2 breaks then we are on the next elf
            2 => {
                elf_idx += 1;
                num_breaks = 0;
                elf_calorie_counts.push(0);
            }
            _ => (),
        }
    }

    // handle end of file as no line break
    elf_calorie_counts[elf_idx] += concat_vec_of_int(&current_food_item);

    elf_calorie_counts
}

fn calculate_day_one_part_one() -> u32 {
    let file_contents = load_file_contents(Path::new("inputs/day1.txt"));

    let elf_calorie_counts = calculate_elf_calorie_counts(&file_contents);

    // find the elf with the most calories!
    *elf_calorie_counts.iter().max().unwrap()
}

fn calculate_day_one_part_two() -> u32 {
    let file_contents = load_file_contents(Path::new("inputs/day1.txt"));

    let mut elf_calorie_counts = calculate_elf_calorie_counts(&file_contents);

    // find the top three elves with the most calories!
    elf_calorie_counts.sort();

    let len = elf_calorie_counts.len();
    elf_calorie_counts[len - 3..].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_value_part_one() {
        assert!(calculate_day_one_part_one() == 74394);
    }

    #[test]
    fn correct_value_part_two() {
        assert!(calculate_day_one_part_two() == 212836);
    }
}
