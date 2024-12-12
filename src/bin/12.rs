use std::{
    collections::VecDeque,
    iter::{Filter, Map},
};

advent_of_code::solution!(12);

#[derive(PartialEq)]
enum VisitState {
    Unvisited,
    Queued,
    Visited,
}

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

    fn from_str(str: &str) -> Grid<char> {
        Grid {
            data: str.lines().flat_map(|s| s.chars()).collect(),
            width: str.lines().next().unwrap().len(),
            height: str.lines().count(),
        }
    }

    fn from_grid<F, U>(&self, map: F) -> Grid<U>
    where
        F: Fn(&T) -> U,
    {
        Grid {
            data: self.data.iter().map(|x| map(x)).collect(),
            width: self.width,
            height: self.height,
        }
    }

    fn at(&self, pos: &(usize, usize)) -> &T {
        let idx = self.width * pos.1 + pos.0;
        &self.data[idx]
    }

    fn at_mut(&mut self, pos: &(usize, usize)) -> &mut T {
        let idx = self.width * pos.1 + pos.0;
        &mut self.data[idx]
    }

    fn out_of_bounds(&self, pos: &(isize, isize)) -> bool {
        pos.0 < 0 || pos.0 >= self.width as isize || pos.1 < 0 || pos.1 >= self.height as isize
    }

    fn iterate(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, val)| ((idx % self.width, idx / self.width), val))
    }

    fn neighbors<'a>(
        &'a self,
        pos: &'a (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        Self::ALL_DIRECTIONS
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
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::<char>::from_str(input);
    let mut visit_grid = Grid::from_grid(&grid, |_| VisitState::Unvisited);

    let price = grid
        .iterate()
        .map(|(pos, val)| {
            if *visit_grid.at(&pos) == VisitState::Unvisited {
                find_region_price(&grid, &mut visit_grid, *val, &pos)
            } else {
                0
            }
        })
        .sum();

    Some(price)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn find_region_price(
    grid: &Grid<char>,
    visit_grid: &mut Grid<VisitState>,
    val: char,
    at: &(usize, usize),
) -> u32 {
    let mut queue = VecDeque::new();
    let mut region_size = 0;
    let mut region_perimeter = 0;

    queue.push_back(*at);

    while let Some(at) = queue.pop_front() {
        region_size += 1;
        region_perimeter += 4;
        *visit_grid.at_mut(&at) = VisitState::Visited;

        grid.neighbors(&at)
            .filter(|at| *grid.at(&at) == val)
            .for_each(|at| match *visit_grid.at(&at) {
                VisitState::Unvisited => {
                    queue.push_back(at);
                    *visit_grid.at_mut(&at) = VisitState::Queued;
                }
                VisitState::Visited => region_perimeter -= 2,
                VisitState::Queued => (),
            });
    }

    region_size * region_perimeter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
