advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let row_num = grid.len();
    let col_num = grid[0].len();

    let xmas = "XMAS";
    let xmas_rev = "SAMX";

    let mut occurrences = 0;

    // horizontal
    for row in grid.iter() {
        occurrences += match_count(&row, xmas);
        occurrences += match_count(&row, xmas_rev);
    }

    // vertical
    for j in 0..col_num {
        let col = (0..row_num).fold(Vec::new(), |mut acc, i| {
            acc.push(grid[i][j]);

            acc
        });

        occurrences += match_count(&col, xmas);
        occurrences += match_count(&col, xmas_rev);
    }

    // diagonal \
    for k in 0..row_num + col_num - 1 {
        let (mut i, mut j) = if k < col_num {
            (0, k)
        } else {
            (k - col_num + 1, 0)
        };

        let mut diag = Vec::new();

        while i < row_num && j < col_num {
            diag.push(grid[i][j]);

            i += 1;
            j += 1;
        }

        occurrences += match_count(&diag, xmas);
        occurrences += match_count(&diag, xmas_rev);
    }

    // diagonal /
    for k in 0..row_num + col_num - 1 {
        let (mut i, mut j) = if k < col_num {
            (0, col_num - 1 - k)
        } else {
            (k - col_num + 1, col_num - 1)
        };

        let mut diag = Vec::new();

        while i < row_num {
            diag.push(grid[i][j]);

            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }

        occurrences += match_count(&diag, xmas);
        occurrences += match_count(&diag, xmas_rev);
    }

    Some(occurrences)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let row_num = grid.len();
    let col_num = grid[0].len();

    let mas = "MAS";
    let mas_rev = "SAM";
    let mas_len = mas.len();

    let mut occurrences = 0;

    for i in 0..row_num - mas_len + 1 {
        for j in 0..col_num - mas_len + 1 {
            let mut diag = Vec::new();

            for k in 0..mas_len {
                diag.push(grid[i + k][j + k]);
            }

            if match_count(&diag, mas) != 1 && match_count(&diag, mas_rev) != 1 {
                continue;
            }

            let mut diag = Vec::new();

            for k in 0..mas_len {
                diag.push(grid[i + mas_len - 1 - k][j + k]);
            }

            if match_count(&diag, mas) != 1 && match_count(&diag, mas_rev) != 1 {
                continue;
            }

            occurrences += 1;
        }
    }

    Some(occurrences)
}

fn match_count(str: &[char], pat: &str) -> u32 {
    str.iter()
        .collect::<String>()
        .match_indices(pat)
        .collect::<Vec<_>>()
        .len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
