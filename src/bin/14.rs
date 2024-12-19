use std::collections::HashMap;

advent_of_code::solution!(14);

struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

#[allow(dead_code)]
struct Grid {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(dims: (usize, usize)) -> Self {
        let width = dims.0;
        let height = dims.1;

        Grid {
            data: vec![' '; width * height],
            width,
            height,
        }
    }

    fn at_mut(&mut self, pos: &(usize, usize)) -> &mut char {
        let idx = self.width * pos.1 + pos.0;
        &mut self.data[idx]
    }

    fn to_str(&self) -> String {
        self.data
            .chunks(self.width)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn has_horizontal_line(&self) -> bool {
        self.data
            .chunks(self.width)
            .map(|chunk| chunk.iter().collect::<String>())
            .any(|line| line.contains(&"*".repeat(30)))
    }
}

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
    let dims = (101, 103);

    let mut robots = Vec::new();

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split(|c: char| !c.is_digit(10) && c != '-')
            .filter_map(|s| s.parse().ok())
            .collect();

        let pos = (nums[0], nums[1]);
        let vel = (nums[2], nums[3]);

        robots.push(Robot { pos, vel });
    }

    for second in 1..1000001 {
        for i in 0..robots.len() {
            robots[i].pos = calculate_board_position(dims, robots[i].pos, robots[i].vel, 1);
        }

        if print_grid_with_robots(dims, &robots, second) {
            break;
        };
    }

    None
}

fn print_grid_with_robots(dims: (i32, i32), robots: &[Robot], second: u32) -> bool {
    let mut grid = Grid::new((dims.0 as usize, dims.1 as usize));

    for robot in robots {
        *grid.at_mut(&(robot.pos.0 as usize, robot.pos.1 as usize)) = '*';
    }

    if grid.has_horizontal_line() {
        println!("After {} seconds:", second);
        println!("{}", grid.to_str());
        println!("");

        return true;
    }

    false
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
