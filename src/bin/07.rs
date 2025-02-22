advent_of_code::solution!(7);

#[derive(Clone)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    fn perform(&self, l: u64, r: u64) -> u64 {
        match self {
            Operation::Add => l + r,
            Operation::Multiply => l * r,
            Operation::Concatenate => {
                let r_len = r.to_string().len() as u32;

                l * 10u64.pow(r_len) + r
            }
        }
    }
}

struct VariationIter<T> {
    size: usize,
    curr: Vec<u8>,
    options: Vec<T>,
    done: bool,
}

impl<T> VariationIter<T> {
    fn new(size: usize, options: Vec<T>) -> Self {
        VariationIter {
            size,
            curr: vec![0; size],
            options,
            done: false,
        }
    }
}

impl<T: Clone> Iterator for VariationIter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self
            .curr
            .iter()
            .map(|&i| self.options[i as usize].clone())
            .collect();

        for i in (0..self.size).rev() {
            self.curr[i] += 1;
            if self.curr[i] < self.options.len() as u8 {
                break;
            } else {
                self.curr[i] = 0;
                if i == 0 {
                    self.done = true;
                }
            }
        }

        Some(result)
    }
}

pub fn part_one_slow(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|line| {
            let (target_num, nums) = parse_line(line);

            let total_combinations = 1 << nums.len() - 1;

            for combination in 0..total_combinations {
                let res = (0..nums.len() - 1).fold(nums[0], |acc, i| {
                    if combination & 1 << i == 0 {
                        acc * nums[i + 1]
                    } else {
                        acc + nums[i + 1]
                    }
                });

                if target_num == res {
                    return target_num;
                }
            }

            0
        })
        .sum();

    Some(sum)
}

pub fn part_two_slow(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|line| {
            let (target_num, nums) = parse_line(line);

            for operations in VariationIter::new(
                nums.len() - 1,
                vec![Operation::Add, Operation::Multiply, Operation::Concatenate],
            ) {
                let res = (0..nums.len() - 1)
                    .fold(nums[0], |acc, i| operations[i].perform(acc, nums[i + 1]));

                if target_num == res {
                    return target_num;
                }
            }

            0
        })
        .sum();

    Some(sum)
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|line| {
            let (res, nums) = parse_line(line);
            let ops = [Operation::Add, Operation::Multiply];

            if expr_exists(res, &nums, &ops, 0) {
                res
            } else {
                0
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|line| {
            let (res, nums) = parse_line(line);
            let ops = [Operation::Add, Operation::Multiply, Operation::Concatenate];

            if expr_exists(res, &nums, &ops, 0) {
                res
            } else {
                0
            }
        })
        .sum();

    Some(sum)
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let tuple = line.split_once(": ").unwrap();

    let res = tuple.0.parse::<u64>().unwrap();
    let nums: Vec<u64> = tuple
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    (res, nums)
}

fn expr_exists(res: u64, nums: &[u64], ops: &[Operation], acc: u64) -> bool {
    if nums.is_empty() {
        return res == acc;
    }

    ops.iter()
        .any(|op| expr_exists(res, &nums[1..], ops, op.perform(acc, nums[0])))
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
        assert_eq!(result, Some(11387));
    }
}
