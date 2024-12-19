use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

advent_of_code::solution!(16);

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

    fn adjacent(&self) -> [Direction; 2] {
        match *self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Right | Direction::Left => [Direction::Up, Direction::Down],
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Grid<char> {
    fn from_str(str: &str) -> Self {
        Grid {
            data: str.lines().flat_map(|s| s.chars()).collect(),
            width: str.lines().next().unwrap().len(),
            height: str.lines().count(),
        }
    }
}

impl<T> Grid<T> {
    const ALL_DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

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

    fn at(&self, pos: (usize, usize)) -> &T {
        let idx = self.width * pos.1 + pos.0;
        &self.data[idx]
    }

    fn at_mut(&mut self, pos: (usize, usize)) -> &mut T {
        let idx = self.width * pos.1 + pos.0;
        &mut self.data[idx]
    }

    fn find<'a>(&'a self, val: T) -> impl Iterator<Item = (usize, usize)> + 'a
    where
        T: PartialEq,
    {
        self.data
            .iter()
            .enumerate()
            .filter(move |&(_, x)| *x == val)
            .map(|(i, _)| ((i % self.width, i / self.width)))
    }

    fn find_first(&self, val: T) -> Option<(usize, usize)>
    where
        T: PartialEq,
    {
        self.find(val).next()
    }

    fn out_of_bounds(&self, pos: (isize, isize)) -> bool {
        pos.0 < 0 || pos.0 >= self.width as isize || pos.1 < 0 || pos.1 >= self.height as isize
    }

    fn neighbors_with_dirs<'a>(
        &'a self,
        pos: &'a (usize, usize),
    ) -> impl Iterator<Item = ((usize, usize), &Direction)> + 'a {
        Self::ALL_DIRECTIONS
            .iter()
            .map(|dir| {
                let dir_val = dir.value();

                (
                    (
                        pos.0 as isize + dir_val.0 as isize,
                        pos.1 as isize + dir_val.1 as isize,
                    ),
                    dir,
                )
            })
            .filter(|(next_pos, _)| !self.out_of_bounds(*next_pos))
            .map(|(next_pos, dir)| ((next_pos.0 as usize, next_pos.1 as usize), dir))
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct GridPosition {
    prev: Option<Box<GridPosition>>,
    pos: (usize, usize),
    dir: Direction,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct State {
    pos: GridPosition,
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input);
    let mut visit_grid = HashMap::new();
    for dir in Grid::<()>::ALL_DIRECTIONS {
        visit_grid.insert(dir, Grid::from_grid(&grid, |_| false));
    }
    let start = GridPosition {
        prev: None,
        pos: grid.find_first('S').unwrap(),
        dir: Direction::Right,
    };
    let end = grid.find_first('E').unwrap();

    find_best_path(&grid, &mut visit_grid, start, end)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input);
    let mut record_grid = HashMap::new();
    for dir in Grid::<()>::ALL_DIRECTIONS {
        record_grid.insert(dir, Grid::from_grid(&grid, |_| None));
    }
    let start = GridPosition {
        prev: None,
        pos: grid.find_first('S').unwrap(),
        dir: Direction::Right,
    };
    let end = grid.find_first('E').unwrap();

    let best_tiles = find_best_tiles(&grid, &mut record_grid, start, end);

    Some(best_tiles.len() as u32)
}

fn find_best_path(
    grid: &Grid<char>,
    visit_grid: &mut HashMap<Direction, Grid<bool>>,
    pos: GridPosition,
    end_pos: (usize, usize),
) -> Option<u32> {
    let mut min_queue = BinaryHeap::new();

    min_queue.push(State { pos, cost: 0 });

    while let Some(state) = min_queue.pop() {
        if state.pos.pos == end_pos {
            return Some(state.cost);
        }

        if *visit_grid.get(&state.pos.dir).unwrap().at(state.pos.pos) == true {
            continue;
        }
        *visit_grid
            .get_mut(&state.pos.dir)
            .unwrap()
            .at_mut(state.pos.pos) = true;

        grid.neighbors_with_dirs(&state.pos.pos)
            .filter(|(next_pos, _)| *grid.at(*next_pos) != '#')
            .for_each(|(next_pos, dir)| {
                if *visit_grid.get(&dir).unwrap().at(next_pos) == false {
                    let next_cost = if *dir == state.pos.dir {
                        1
                    } else if state.pos.dir.adjacent().contains(dir) {
                        1001
                    } else {
                        2001
                    };

                    min_queue.push(State {
                        pos: GridPosition {
                            prev: None,
                            pos: next_pos,
                            dir: dir.clone(),
                        },
                        cost: state.cost + next_cost,
                    });
                }
            });
    }

    None
}

fn find_best_tiles(
    grid: &Grid<char>,
    visit_grid: &mut HashMap<Direction, Grid<Option<u32>>>,
    pos: GridPosition,
    end_pos: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut min_queue = BinaryHeap::new();

    min_queue.push(State { pos, cost: 0 });

    let mut best_cost = None;
    let mut best_tiles = HashSet::new();

    while let Some(state) = min_queue.pop() {
        if state.pos.pos == end_pos {
            best_cost.get_or_insert(state.cost);

            if state.cost == best_cost.unwrap() {
                let mut curr_pos = Box::new(state.pos);
                best_tiles.insert(curr_pos.pos);

                while let Some(prev_pos) = curr_pos.prev {
                    curr_pos = prev_pos;
                    best_tiles.insert(curr_pos.pos);
                }
            }

            continue;
        }

        let recorded_cost = visit_grid.get(&state.pos.dir).unwrap().at(state.pos.pos);

        if let Some(cost) = recorded_cost {
            if *cost < state.cost {
                continue;
            }
        }

        *visit_grid
            .get_mut(&state.pos.dir)
            .unwrap()
            .at_mut(state.pos.pos) = Some(state.cost);

        grid.neighbors_with_dirs(&state.pos.pos)
            .filter(|(next_pos, _)| *grid.at(*next_pos) != '#')
            .for_each(|(next_pos, dir)| {
                let next_cost = if *dir == state.pos.dir {
                    1
                } else if state.pos.dir.adjacent().contains(dir) {
                    1001
                } else {
                    2001
                };

                min_queue.push(State {
                    pos: GridPosition {
                        prev: Some(Box::new(state.pos.clone())),
                        pos: next_pos,
                        dir: dir.clone(),
                    },
                    cost: state.cost + next_cost,
                });
            });
    }

    best_tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
