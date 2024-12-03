advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let byte_str = input.as_bytes();
    let mut sum = 0;

    for (i, _) in input.match_indices("mul") {
        if let Some(res) = handle_mul(byte_str, i) {
            sum += res;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let byte_str = input.as_bytes();
    let mut sum = 0;
    let mut should_handle_mul = true;

    let mut vec: Vec<_> = input
        .match_indices("mul")
        .chain(input.match_indices("do()"))
        .chain(input.match_indices("don't()"))
        .collect();

    vec.sort_by_key(|&(i, _)| i);

    for (i, command) in vec {
        match command {
            "do()" => should_handle_mul = true,
            "don't()" => should_handle_mul = false,
            "mul" => {
                if !should_handle_mul {
                    continue;
                }
                if let Some(res) = handle_mul(byte_str, i) {
                    sum += res;
                }
            }
            _ => continue,
        }
    }

    Some(sum)
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

fn handle_mul(byte_str: &[u8], i: usize) -> Option<u32> {
    let mut curr = i + 3;

    if byte_str[curr] != b'(' {
        return None;
    }
    curr += 1;

    let to = find_num_end_pos(byte_str, curr);

    if to < curr || to - curr > 3 {
        return None;
    }

    let num_1 = consume_num(byte_str, &mut curr, to);

    if byte_str[curr] != b',' {
        return None;
    }
    curr += 1;

    let to = find_num_end_pos(byte_str, curr);

    if to < curr || to - curr > 3 {
        return None;
    }

    let num_2 = consume_num(byte_str, &mut curr, to);

    if byte_str[curr] != b')' {
        return None;
    }

    Some(num_1 * num_2)
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
