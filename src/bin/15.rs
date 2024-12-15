advent_of_code::solution!(15);

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
}

impl<T> Grid<T> {
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

    fn iterate(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, val)| ((idx % self.width, idx / self.width), val))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, dirs) = input.split_once("\n\n").unwrap();

    let mut grid = Grid::from_str(grid);
    let mut robot_pos = grid.find_first('@').unwrap();
    *grid.at_mut(robot_pos) = '.';

    for dir in dirs
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| Direction::from_char(x).unwrap())
    {
        let next_pos = toward(robot_pos, &dir);

        match grid.at(next_pos) {
            '#' => (),
            '.' => robot_pos = next_pos,
            'O' => {
                if move_boxes(&mut grid, next_pos, &dir) {
                    robot_pos = next_pos
                }
            }
            _ => panic!(),
        }
    }

    let res = grid
        .iterate()
        .map(|(pos, val)| {
            if *val == 'O' {
                (pos.0 + pos.1 * 100) as u32
            } else {
                0
            }
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn toward(pos: (usize, usize), dir: &Direction) -> (usize, usize) {
    let dir_val = dir.value();

    let dir_val = (dir_val.0 as isize, dir_val.1 as isize);
    let pos = (pos.0 as isize, pos.1 as isize);

    let next_pos = (pos.0 + dir_val.0, pos.1 + dir_val.1);

    (next_pos.0 as usize, next_pos.1 as usize)
}

fn move_boxes(grid: &mut Grid<char>, at: (usize, usize), dir: &Direction) -> bool {
    let next_at = toward(at, &dir);

    match grid.at(next_at) {
        '#' => false,
        '.' => {
            *grid.at_mut(at) = '.';
            *grid.at_mut(next_at) = 'O';

            true
        }
        'O' => {
            if move_boxes(grid, next_at, dir) {
                *grid.at_mut(at) = '.';
                *grid.at_mut(next_at) = 'O';

                true
            } else {
                false
            }
        }
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
