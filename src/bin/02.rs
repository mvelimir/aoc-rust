advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .map(|x| {
            let number_vec: Vec<_> = x
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let positive = number_vec[1] - number_vec[0] > 0;

            number_vec
                .windows(2)
                .map(|x| x[1] - x[0])
                .filter(|x| x.abs() >= 1 && x.abs() <= 3)
                .filter(|x| if positive { *x > 0 } else { *x <= 0 })
                .count()
                == number_vec.len() - 1
        })
        .filter(|x| *x == true)
        .count() as u32;

    Some(count)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
