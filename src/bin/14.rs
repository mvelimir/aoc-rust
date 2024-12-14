use std::collections::HashMap;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let dims = (101, 103);

    let mut quadrant_count_map: HashMap<u8, u32> = HashMap::new();

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split(|c: char| !c.is_digit(10) && c != '-')
            .filter_map(|s| s.parse().ok())
            .collect();

        let pos = (nums[0], nums[1]);
        let vel = (nums[2], nums[3]);

        let final_pos = calculate_board_position(dims, pos, vel, 100);
        if let Some(quadrant) = calculate_quandrant(dims, final_pos) {
            *quadrant_count_map.entry(quadrant).or_insert(0) += 1;
        }
    }

    let prod = quadrant_count_map.values().fold(1, |acc, x| acc * x);

    Some(prod)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn calculate_board_position(
    dims: (i32, i32),
    pos: (i32, i32),
    vel: (i32, i32),
    seconds: i32,
) -> (i32, i32) {
    let final_pos = (
        (pos.0 + vel.0 * seconds).rem_euclid(dims.0),
        (pos.1 + vel.1 * seconds).rem_euclid(dims.1),
    );

    final_pos
}

fn calculate_quandrant(dims: (i32, i32), pos: (i32, i32)) -> Option<u8> {
    let center = (dims.0 / 2, dims.1 / 2);

    match pos {
        (x, y) if x > center.0 && y < center.1 => Some(0),
        (x, y) if x < center.0 && y < center.1 => Some(1),
        (x, y) if x < center.0 && y > center.1 => Some(2),
        (x, y) if x > center.0 && y > center.1 => Some(3),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
