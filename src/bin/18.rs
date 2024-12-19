use std::collections::VecDeque;

advent_of_code::solution!(18);

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
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
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    const ALL_DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    fn initialize(width: usize, height: usize, default: T) -> Self
    where
        T: Clone,
    {
        Grid {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    fn at(&self, pos: (usize, usize)) -> &T {
        let idx = self.width * pos.1 + pos.0;
        &self.data[idx]
    }

    fn at_mut(&mut self, pos: (usize, usize)) -> &mut T {
        let idx = self.width * pos.1 + pos.0;
        &mut self.data[idx]
    }

    fn out_of_bounds(&self, pos: (isize, isize)) -> bool {
        pos.0 < 0 || pos.0 >= self.width as isize || pos.1 < 0 || pos.1 >= self.height as isize
    }

    fn neighbors<'a>(&'a self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + 'a {
        Self::ALL_DIRECTIONS
            .iter()
            .map(move |dir| {
                let dir = dir.value();

                (
                    pos.0 as isize + dir.0 as isize,
                    pos.1 as isize + dir.1 as isize,
                )
            })
            .filter(|next_pos| !self.out_of_bounds(*next_pos))
            .map(|next_pos| (next_pos.0 as usize, next_pos.1 as usize))
    }
}

struct Position {
    pos: (usize, usize),
    cost: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let start = (0, 0);
    let end = (70, 70);
    let mut grid = Grid::initialize(end.0 + 1, end.1 + 1, '.');

    input
        .lines()
        .take(1024)
        .map(|x| x.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .for_each(|pos| *grid.at_mut(pos) = '#');

    find_shortest_path_len(&mut grid, start, end)
}

pub fn part_two(input: &str) -> Option<String> {
    let start = (0, 0);
    let end = (70, 70);
    let mut grid = Grid::initialize(end.0 + 1, end.1 + 1, '.');

    for pos in input
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
    {
        *grid.at_mut(pos) = '#';

        if find_shortest_path_len(&mut grid.clone(), start, end).is_none() {
            return Some(format!("{},{}", pos.0, pos.1));
        }
    }

    None
}

fn find_shortest_path_len(
    grid: &mut Grid<char>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<u32> {
    let mut queue = VecDeque::new();

    queue.push_back(Position {
        pos: start,
        cost: 0,
    });

    while let Some(pos) = queue.pop_front() {
        if pos.pos == end {
            return Some(pos.cost);
        }

        for at in grid.neighbors(pos.pos).collect::<Vec<_>>() {
            if *grid.at(at) == '.' {
                queue.push_back(Position {
                    pos: at,
                    cost: pos.cost + 1,
                });
                *grid.at_mut(at) = '#';
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("6,1")));
    }
}
