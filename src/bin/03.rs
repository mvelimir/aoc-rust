advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let byte_str = input.as_bytes();
    let mut sum = 0;

    for (i, _) in input.match_indices("mul") {
        let mut curr = i + 3;

        if byte_str[curr] != b'(' {
            continue;
        }
        curr += 1;

        let to = find_num_end_pos(byte_str, curr);

        if to < curr || to - curr > 3 {
            continue;
        }

        let num_1 = consume_num(byte_str, &mut curr, to);

        if byte_str[curr] != b',' {
            continue;
        }
        curr += 1;

        let to = find_num_end_pos(byte_str, curr);

        if to < curr || to - curr > 3 {
            continue;
        }

        let num_2 = consume_num(byte_str, &mut curr, to);

        if byte_str[curr] != b')' {
            continue;
        }

        sum += num_1 * num_2;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn find_num_end_pos(byte_str: &[u8], from: usize) -> usize {
    let mut to = from - 1;

    while to < byte_str.len() - 2 && byte_str[to + 1].is_ascii_digit() {
        to += 1;
    }

    to
}

fn consume_num(byte_str: &[u8], curr: &mut usize, to: usize) -> u32 {
    let num = std::str::from_utf8(&byte_str[*curr..=to])
        .unwrap()
        .parse::<u32>()
        .unwrap();
    *curr = to + 1;

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
