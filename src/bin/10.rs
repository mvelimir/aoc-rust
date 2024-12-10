use std::collections::HashSet;

advent_of_code::solution!(10);

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn value(&self) -> (i8, i8) {
        match *self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

#[derive(Clone)]
struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_str(str: &str) -> Self {
        Grid {
            data: str
                .lines()
                .flat_map(|s| s.chars())
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
            width: str.lines().next().unwrap().len(),
            height: str.lines().count(),
        }
    }

    fn at(&self, pos: &(usize, usize)) -> u8 {
        let idx = self.width * pos.1 + pos.0;
        self.data[idx]
    }

    fn find(&self, num: u8) -> Vec<(usize, usize)> {
        self.data
            .iter()
            .enumerate()
            .filter(|&(_, x)| *x == num)
            .map(|(i, _)| ((i % self.width, i / self.width)))
            .collect()
    }

    fn out_of_bounds(&self, pos: &(isize, isize)) -> bool {
        pos.0 < 0 || pos.0 >= self.width as isize || pos.1 < 0 || pos.1 >= self.height as isize
    }

    fn possible_next_positions(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
        .map(|dir| {
            let dir = dir.value();

            (
                pos.0 as isize + dir.0 as isize,
                pos.1 as isize + dir.1 as isize,
            )
        })
        .filter(|next_pos| !self.out_of_bounds(next_pos))
        .map(|next_pos| (next_pos.0 as usize, next_pos.1 as usize))
        .filter(|next_pos| self.at(next_pos) == self.at(pos) + 1)
        .collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input);

    let score = grid
        .find(0)
        .into_iter()
        .map(|trailhead| {
            let mut possible_destinations = HashSet::new();

            find_trailhead_score(&grid, &trailhead, &mut possible_destinations);

            possible_destinations.len() as u32
        })
        .sum();

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn find_trailhead_score(grid: &Grid, pos: &(usize, usize), acc: &mut HashSet<String>) {
    if grid.at(pos) == 9 {
        acc.insert(format!("{},{}", pos.0, pos.1));

        return;
    }

    grid.possible_next_positions(&pos)
        .iter()
        .for_each(|next_pos| find_trailhead_score(grid, next_pos, acc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
