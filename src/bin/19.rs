use std::collections::HashMap;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (towels, patterns) = input.split_once("\n\n").unwrap();

    let towels = towels.split(", ").collect::<Vec<_>>();

    let mut pattern_possibility = HashMap::new();

    let res = patterns
        .lines()
        .filter(|pattern| is_pattern_possible(pattern, &towels, &mut pattern_possibility))
        .count();

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn is_pattern_possible<'a>(
    pattern: &'a str,
    towels: &[&str],
    pattern_possibility: &mut HashMap<String, bool>,
) -> bool {
    if pattern.is_empty() {
        return true;
    }

    if let Some(res) = pattern_possibility.get(pattern) {
        return *res;
    }

    for i in 1..pattern.len() + 1 {
        for &towel in towels.iter() {
            if pattern[0..i] == *towel {
                if is_pattern_possible(&pattern[i..], towels, pattern_possibility) {
                    pattern_possibility.entry(String::from(pattern)).or_insert(true);

                    return true;
                } else {
                    pattern_possibility.entry(String::from(&pattern[i..])).or_insert(false);
                }
            }
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
