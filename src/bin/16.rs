use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
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
    fn from_char(char: char) -> Option<Self> {
        match char {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }

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

    fn to_str(&self) -> String {
        self.data
            .chunks(self.width)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
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
        pos: grid.find_first('S').unwrap(),
        dir: Direction::Right,
    };
    let end = grid.find_first('E').unwrap();

    find_best_path(&grid, &mut visit_grid, start, end)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input);
    let mut visit_grid = HashMap::new();
    for dir in Grid::<()>::ALL_DIRECTIONS {
        visit_grid.insert(dir, Grid::from_grid(&grid, |_| false));
    }
    let start = GridPosition {
        pos: grid.find_first('S').unwrap(),
        dir: Direction::Right,
    };
    let end = grid.find_first('E').unwrap();

    find_best_spots(&grid, &mut visit_grid, start, end)
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

fn find_best_distance(
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
