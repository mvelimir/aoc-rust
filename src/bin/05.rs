use std::{cmp::Ordering, collections::HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    calculate_result(input, |update, less_than_set| {
        if update
            .windows(2)
            .all(|w| less_than_set.contains(format!("{}|{}", w[0], w[1]).as_str()))
        {
            result += update[update.len() / 2];
        }
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    calculate_result(input, |update, less_than_set| {
        if !update
            .windows(2)
            .all(|w| less_than_set.contains(format!("{}|{}", w[0], w[1]).as_str()))
        {
            update.sort_by(|a, b| {
                if less_than_set.contains(format!("{}|{}", a, b).as_str()) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            result += update[update.len() / 2];
        }
    });

    Some(result)
}

fn calculate_result<F>(input: &str, mut calculation: F) -> ()
where
    F: FnMut(&mut [u32], &HashSet<&str>),
{
    let (ordering_rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let mut less_than_set = HashSet::new();

    for line in ordering_rules_str.lines() {
        less_than_set.insert(line);
    }

    for line in updates_str.lines() {
        let mut update = line
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();

        calculation(&mut update, &less_than_set);
    }
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
        assert_eq!(result, Some(123));
    }
}
