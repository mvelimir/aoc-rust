advent_of_code::solution!(6);

#[derive(Hash, PartialEq, Eq)]
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

    fn rotate_right(&mut self) -> () {
        match *self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }
}

struct Grid {
    data: String,
    width: usize,
    height: usize,
}

impl Grid {
    fn at(&self, pos: &(usize, usize)) -> char {
        let idx = self.width * pos.1 + pos.0;
        self.data.chars().nth(idx).unwrap()
    }

    fn replace_at(&mut self, pos: &(usize, usize), char: char) -> () {
        let idx = self.width * pos.1 + pos.0;
        self.data.replace_range(idx..idx + 1, &char.to_string());
    }

    fn find(&self, char: char) -> (usize, usize) {
        let idx = self.data.find(char).unwrap();
        (idx % self.width, idx / self.width)
    }

    fn out_of_bounds(&self, pos: (i32, i32)) -> bool {
        pos.0 < 0 || pos.0 >= self.width as i32 || pos.1 < 0 || pos.1 >= self.height as i32
    }
}

struct Guard {
    pos: (usize, usize),
    dir: Direction,
    in_grid: bool,
}

impl Guard {
    fn next_pos(&self) -> (i32, i32) {
        let dir = self.dir.value();
        (
            self.pos.0 as i32 + dir.0 as i32,
            self.pos.1 as i32 + dir.1 as i32,
        )
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid {
        data: input.lines().collect(),
        width: input.lines().next().unwrap().len(),
        height: input.lines().count(),
    };

    let mut guard = Guard {
        pos: grid.find('^'),
        dir: Direction::Up,
        in_grid: true,
    };

    let mut unique_positions = 0;

    grid.replace_at(&guard.pos, 'X');
    unique_positions += 1;

    while guard.in_grid {
        let next_pos = guard.next_pos();

        if grid.out_of_bounds(next_pos) {
            guard.in_grid = false;
            continue;
        }

        let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

        match grid.at(&next_pos) {
            '.' => {
                guard.pos = next_pos;
                grid.replace_at(&guard.pos, 'X');
                unique_positions += 1;
            }
            'X' => {
                guard.pos = next_pos;
            }
            '#' => {
                guard.dir.rotate_right();
            }
            _ => panic!(),
        };
    }

    Some(unique_positions)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
