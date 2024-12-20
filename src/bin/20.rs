use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(20);

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

    fn neighbors_with_dirs<'a>(
        &'a self,
        pos: (usize, usize),
    ) -> impl Iterator<Item = ((usize, usize), &Direction)> + 'a {
        Self::ALL_DIRECTIONS
            .iter()
            .map(move |dir| {
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

#[derive(Hash, PartialEq, Eq, Clone)]
struct Position {
    pos: (usize, usize),
    cost: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::from_str(input);

    let start = grid.find_first('S').unwrap();
    let end = grid.find_first('E').unwrap();

    let from_start = find_dist_from_start(&grid, start, end);

    let res = find_shortcut_lenghts(&mut grid, &from_start, start, end)
        .into_iter()
        .filter(|&x| x >= 100)
        .count();

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::from_str(input);

    let start = grid.find_first('S').unwrap();
    let end = grid.find_first('E').unwrap();

    let from_start = find_dist_from_start(&grid, start, end);

    let res = find_20ps_shortcut_lenghts(&mut grid, &from_start, start, end)
        .into_iter()
        .filter(|&x| x >= 100)
        .count();

    Some(res as u32)
}

fn find_dist_from_start(
    grid: &Grid<char>,
    start: (usize, usize),
    end: (usize, usize),
) -> HashMap<(usize, usize), u32> {
    let mut from_start = HashMap::new();
    let mut current = start;
    let mut dist = 0;

    loop {
        from_start.insert(current, dist);

        if current == end {
            break;
        }

        current = grid
            .neighbors(current)
            .find(|x| *grid.at(*x) != '#' && !from_start.contains_key(x))
            .unwrap();
        dist += 1;
    }

    from_start
}

fn find_shortcut_lenghts(
    grid: &mut Grid<char>,
    from_start: &HashMap<(usize, usize), u32>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<u32> {
    let mut shortcut_lengths = vec![];
    let mut current = start;

    loop {
        *grid.at_mut(current) = '#';

        for (pos, dir) in grid
            .neighbors_with_dirs(current)
            .filter(|(x, _)| *grid.at(*x) == '#')
        {
            let next_pos = toward(pos, dir);
            if grid.out_of_bounds(next_pos) {
                continue;
            }

            let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

            if *grid.at(next_pos) != '#' {
                let save = *from_start.get(&next_pos).unwrap() as i32
                    - *from_start.get(&current).unwrap() as i32
                    - 2;

                if save >= 0 {
                    shortcut_lengths.push(save as u32);
                }
            }
        }

        if current == end {
            break;
        }

        current = grid
            .neighbors(current)
            .find(|x| *grid.at(*x) != '#')
            .unwrap();
    }

    shortcut_lengths
}

fn find_20ps_shortcut_lenghts(
    grid: &mut Grid<char>,
    from_start: &HashMap<(usize, usize), u32>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<u32> {
    let mut shortcut_lengths = vec![];
    let mut current = start;

    loop {
        *grid.at_mut(current) = '#';

        let shortcuts = find_shortcuts_from(&grid, current, 20);

        for next_pos in shortcuts {
            let save = *from_start.get(&next_pos.pos).unwrap() as i32
                - *from_start.get(&current).unwrap() as i32
                - next_pos.cost as i32;

            if save >= 0 {
                shortcut_lengths.push(save as u32);
            }
        }

        if current == end {
            break;
        }

        current = grid
            .neighbors(current)
            .find(|x| *grid.at(*x) != '#')
            .unwrap();
    }

    shortcut_lengths
}

fn find_shortcuts_from(
    grid: &Grid<char>,
    start: (usize, usize),
    max_dist: u32,
) -> HashSet<Position> {
    let mut res = HashSet::new();
    let mut queue = VecDeque::new();
    let mut queued = HashSet::new();

    queue.push_back(Position {
        pos: start,
        cost: 0,
    });

    while let Some(pos) = queue.pop_front() {
        if pos.cost > max_dist {
            break;
        }

        if *grid.at(pos.pos) != '#' {
            res.insert(pos.clone());
        }

        for at in grid.neighbors(pos.pos) {
            if !queued.contains(&at) {
                queue.push_back(Position {
                    pos: at,
                    cost: pos.cost + 1,
                });

                queued.insert(at);
            }
        }
    }

    res
}

fn toward(pos: (usize, usize), dir: &Direction) -> (isize, isize) {
    let dir_val = dir.value();

    let dir_val = (dir_val.0 as isize, dir_val.1 as isize);
    let pos = (pos.0 as isize, pos.1 as isize);

    (pos.0 + dir_val.0, pos.1 + dir_val.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
