advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|x| {
            let tuple = x.split_once(": ").unwrap();

            let target_num = tuple.0.parse::<u64>().unwrap();
            let nums: Vec<u64> = tuple
                .1
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            let total_combinations = 1 << nums.len() - 1;

            for combination in 0..total_combinations {
                let res = (0..nums.len() - 1)
                    .enumerate()
                    .fold(nums[0], |acc, (i, bit)| {
                        if combination & 1 << bit == 0 {
                            acc * nums[i + 1]
                        } else {
                            acc + nums[i + 1]
                        }
                    });

                if target_num == res {
                    return Some(target_num);
                }
            }

            None
        })
        .fold(Some(0), |acc, x| {
            acc.map(|acc| x.map_or(acc, |val| acc + val))
        })
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
