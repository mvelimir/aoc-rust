advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .split("\n\n")
        .map(|str| {
            let nums = parse_numbers(str);
            if let Some((a, b)) =
                solve_linear_system(nums[0], nums[2], nums[4], nums[1], nums[3], nums[5])
            {
                a * 3 + b
            } else {
                0
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .split("\n\n")
        .map(|str| {
            let nums = parse_numbers(str);
            if let Some((a, b)) = solve_linear_system(
                nums[0],
                nums[2],
                nums[4] + 10000000000000,
                nums[1],
                nums[3],
                nums[5] + 10000000000000,
            ) {
                a * 3 + b
            } else {
                0
            }
        })
        .sum();

    Some(sum)
}

fn parse_numbers(input: &str) -> Vec<u64> {
    let mut numbers = Vec::new();
    let mut curr_number = String::new();

    for ch in input.chars() {
        if ch.is_digit(10) {
            curr_number.push(ch);
        } else if !curr_number.is_empty() {
            if let Ok(num) = curr_number.parse::<u64>() {
                numbers.push(num);
            }
            curr_number.clear();
        }
    }

    if !curr_number.is_empty() {
        if let Ok(num) = curr_number.parse::<u64>() {
            numbers.push(num);
        }
    }

    numbers
}

fn solve_linear_system(a1: u64, b1: u64, c1: u64, a2: u64, b2: u64, c2: u64) -> Option<(u64, u64)> {
    let a1 = a1 as i64;
    let b1 = b1 as i64;
    let c1 = c1 as i64;
    let a2 = a2 as i64;
    let b2 = b2 as i64;
    let c2 = c2 as i64;

    let det = a1 * b2 - a2 * b1;

    if det == 0 {
        return None;
    }

    let x_num = c1 * b2 - c2 * b1;
    let y_num = a1 * c2 - a2 * c1;

    if x_num % det != 0 || y_num % det != 0 {
        return None;
    }

    let x = x_num / det;
    let y = y_num / det;

    Some((x as u64, y as u64))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
