use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let [ordering_rules_str, updates_str] = input.split("\n\n").collect::<Vec<_>>()[..] else {
        panic!();
    };

    let mut result = 0;
    let mut less_than_set = HashSet::new();

    for line in ordering_rules_str.lines() {
        let [l, r] = line
            .split("|")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>()[..]
        else {
            panic!();
        };

        less_than_set.insert((l, r));
    }

    for line in updates_str.lines() {
        let update = line
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();

        if update
            .windows(2)
            .all(|w| less_than_set.contains(&(w[0], w[1])))
        {
            result += update[update.len() / 2];
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
