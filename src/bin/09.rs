use advent_of_code::template::aoc_cli::check;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut vec: Vec<u64> = input.lines().next().unwrap().chars().map(|x| x.to_digit(10).unwrap() as u64).collect();
    let mut checksum = 0;
    let last_idx = vec.len() - 1;

    let mut start_idx = 0;
    let mut empty_idx = 1;
    let mut end_idx = last_idx - last_idx % 2;
    let mut curr_pos = 0;
    let mut filling_empty = false;

    while start_idx <= end_idx {
        if !filling_empty {
            checksum += start_idx as u64 / 2 * sum(curr_pos, curr_pos + vec[start_idx]);
            curr_pos += vec[start_idx];
            start_idx += 2;

            filling_empty = true;
            continue;
        }

        if vec[empty_idx] < vec[end_idx] {
            checksum += end_idx as u64 / 2 * sum(curr_pos, curr_pos + vec[empty_idx]);
            curr_pos += vec[empty_idx];
            vec[end_idx] -= vec[empty_idx];
            empty_idx += 2;
            filling_empty = false;
        } else if vec[empty_idx] > vec[end_idx] {
            checksum += end_idx as u64 / 2 * sum(curr_pos, curr_pos + vec[end_idx]);
            curr_pos += vec[end_idx];
            vec[empty_idx] -= vec[end_idx];
            end_idx -= 2;
        } else {
            checksum += end_idx as u64 / 2 * sum(curr_pos, curr_pos + vec[empty_idx]);
            curr_pos += vec[empty_idx];
            end_idx -= 2;
            empty_idx += 2;
            filling_empty = false;
        }
    }

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn sum(from: u64, to: u64) -> u64 {
    to * (to - 1) / 2 + from * (from - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
