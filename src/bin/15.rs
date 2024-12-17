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

    fn to_str(&self) -> String {
        self.data
            .chunks(self.width)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
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
    let (grid, dirs) = input.split_once("\n\n").unwrap();

    let grid = grid.chars().fold(String::new(), |acc, x| match x {
        '#' => acc + "##",
        '@' => acc + "@.",
        'O' => acc + "[]",
        '.' => acc + "..",
        '\n' => acc + "\n",
        _ => panic!(),
    });

    let mut grid = Grid::from_str(&grid);
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
            '[' | ']' => {
                if move_robot(&mut grid, next_pos, &dir) {
                    robot_pos = next_pos
                }
            }
            _ => panic!(),
        }
    }

    let res = grid
        .iterate()
        .map(|(pos, val)| {
            if *val == '[' {
                (pos.0 + pos.1 * 100) as u32
            } else {
                0
            }
        })
        .sum();

    Some(res)
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

fn move_robot(grid: &mut Grid<char>, at: (usize, usize), dir: &Direction) -> bool {
    let (at_start, at_end) = if *grid.at(at) == '[' {
        (at, toward(at, &Direction::Right))
    } else {
        (toward(at, &Direction::Left), at)
    };

    match dir {
        Direction::Left => move_wide_boxes_left_right(grid, at_end, at_start, dir),
        Direction::Right => move_wide_boxes_left_right(grid, at_start, at_end, dir),
        Direction::Up | Direction::Down => {
            if can_move_wide_boxes_up_down(grid, at_start, at_end, dir) {
                move_wide_boxes_up_down(grid, at_start, at_end, dir);

                true
            } else {
                false
            }
        }
    }
}

fn move_wide_boxes_left_right(
    grid: &mut Grid<char>,
    at_start: (usize, usize),
    at_end: (usize, usize),
    dir: &Direction,
) -> bool {
    let next_at = toward(at_end, &dir);

    match grid.at(next_at) {
        '#' => false,
        '.' => {
            *grid.at_mut(next_at) = *grid.at(at_end);
            *grid.at_mut(at_end) = *grid.at(at_start);
            *grid.at_mut(at_start) = '.';

            true
        }
        char if char == grid.at(at_start) => {
            let at_start_1 = next_at;
            let at_end_1 = toward(next_at, &dir);

            if move_wide_boxes_left_right(grid, at_start_1, at_end_1, dir) {
                *grid.at_mut(next_at) = *grid.at(at_end);
                *grid.at_mut(at_end) = *grid.at(at_start);
                *grid.at_mut(at_start) = '.';

                true
            } else {
                false
            }
        }
        _ => panic!(),
    }
}

fn can_move_wide_boxes_up_down(
    grid: &Grid<char>,
    at_start: (usize, usize),
    at_end: (usize, usize),
    dir: &Direction,
) -> bool {
    let next_at_1 = toward(at_start, &dir);
    let next_at_2 = toward(at_end, &dir);

    match (grid.at(next_at_1), grid.at(next_at_2)) {
        ('#', _) | (_, '#') => false,
        ('.', '.') => true,
        ('[', ']') => can_move_wide_boxes_up_down(grid, next_at_1, next_at_2, dir),
        (']', '.') => {
            let next_at_2 = next_at_1;
            let next_at_1 = toward(next_at_1, &Direction::Left);

            can_move_wide_boxes_up_down(grid, next_at_1, next_at_2, dir)
        }
        ('.', '[') => {
            let next_at_1 = next_at_2;
            let next_at_2 = toward(next_at_1, &Direction::Right);

            can_move_wide_boxes_up_down(grid, next_at_1, next_at_2, dir)
        }
        (']', '[') => {
            {
                let next_at_2 = next_at_1;
                let next_at_1 = toward(next_at_1, &Direction::Left);

                if !can_move_wide_boxes_up_down(grid, next_at_1, next_at_2, dir) {
                    return false;
                }
            }

            let next_at_1 = next_at_2;
            let next_at_2 = toward(next_at_1, &Direction::Right);

            can_move_wide_boxes_up_down(grid, next_at_1, next_at_2, dir)
        }
        _ => panic!(),
    }
}

fn move_wide_boxes_up_down(
    grid: &mut Grid<char>,
    at_start: (usize, usize),
    at_end: (usize, usize),
    dir: &Direction,
) -> () {
    let next_at_1 = toward(at_start, &dir);
    let next_at_2 = toward(at_end, &dir);

    match (grid.at(next_at_1), grid.at(next_at_2)) {
        ('.', '.') => (),
        ('[', ']') => move_wide_boxes_up_down(grid, next_at_1, next_at_2, dir),
        (']', '.') => {
            let next_at_2 = next_at_1;
            let next_at_1 = toward(next_at_1, &Direction::Left);

            move_wide_boxes_up_down(grid, next_at_1, next_at_2, dir);
        }
        ('.', '[') => {
            let next_at_1 = next_at_2;
            let next_at_2 = toward(next_at_1, &Direction::Right);

            move_wide_boxes_up_down(grid, next_at_1, next_at_2, dir);
        }
        (']', '[') => {
            {
                let next_at_2 = next_at_1;
                let next_at_1 = toward(next_at_1, &Direction::Left);

                move_wide_boxes_up_down(grid, next_at_1, next_at_2, dir);
            }

            let next_at_1 = next_at_2;
            let next_at_2 = toward(next_at_1, &Direction::Right);

            move_wide_boxes_up_down(grid, next_at_1, next_at_2, dir);
        }
        _ => panic!(),
    }

    *grid.at_mut(next_at_1) = *grid.at(at_start);
    *grid.at_mut(next_at_2) = *grid.at(at_end);
    *grid.at_mut(at_start) = '.';
    *grid.at_mut(at_end) = '.';
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
        assert_eq!(result, Some(9021));
    }
}
