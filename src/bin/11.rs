use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .map(|num| transform_stone(num, 25))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut memoization = HashMap::new();
    let sum = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .map(|num| transform_stone_memoization(num, 75, &mut memoization))
        .sum();

    Some(sum)
}

fn transform_stone(num: u64, blinks: u8) -> u64 {
    if blinks == 0 {
        1
    } else {
        match num {
            0 => transform_stone(1, blinks - 1),
            _ if (num as f32).log10().floor() as u32 % 2 == 1 => {
                let digits = (num as f32).log10().floor() as u32 + 1;
                let divisor = 10u64.pow(digits / 2);

                transform_stone(num / divisor, blinks - 1)
                    + transform_stone(num % divisor, blinks - 1)
            }
            _ => transform_stone(num * 2024, blinks - 1),
        }
    }
}

fn transform_stone_memoization(num: u64, blinks: u8, memoization: &mut HashMap<String, u64>) -> u64 {
    if blinks == 0 {
        1
    } else {
        if let Some(res) = memoization.get(&format!("{},{}", num, blinks)) {
            return *res;
        }

        match num {
            0 => {
                let res = transform_stone_memoization(1, blinks - 1, memoization);
                memoization.entry(format!("{},{}", 1, blinks - 1)).or_insert(res);

                res
            },
            _ if (num as f32).log10().floor() as u32 % 2 == 1 => {
                let digits = (num as f32).log10().floor() as u32 + 1;
                let divisor = 10u64.pow(digits / 2);

                let res_1 = transform_stone_memoization(num / divisor, blinks - 1, memoization);
                memoization.entry(format!("{},{}", num / divisor, blinks - 1)).or_insert(res_1);

                let res_2 = transform_stone_memoization(num % divisor, blinks - 1, memoization);
                memoization.entry(format!("{},{}", num % divisor, blinks - 1)).or_insert(res_2);

                res_1 + res_2
            }
            _ => {
                let res = transform_stone_memoization(num * 2024, blinks - 1, memoization);
                memoization.entry(format!("{},{}", num * 2024, blinks - 1)).or_insert(res);

                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
