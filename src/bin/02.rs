advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .map(|x| {
            let number_vec: Vec<_> = x
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            check_report(number_vec)
        })
        .filter(|x| *x == true)
        .count() as u32;

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .map(|x| {
            let number_vec: Vec<_> = x
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let vec_iter = std::iter::once(number_vec.clone()).chain(
                (0..number_vec.len()).into_iter().map(|i| {
                    let mut vec = number_vec.clone();

                    vec.remove(i);

                    vec
                }),
            );

            vec_iter.map(check_report).find(|x| *x).is_some()
        })
        .filter(|x| *x == true)
        .count() as u32;

    Some(count)
}

fn check_report(vec: Vec<i32>) -> bool {
    let positive = vec[1] - vec[0] > 0;

    vec.windows(2)
        .map(|x| x[1] - x[0])
        .filter(|x| x.abs() >= 1 && x.abs() <= 3)
        .filter(|x| if positive { *x > 0 } else { *x <= 0 })
        .count()
        == vec.len() - 1
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
        assert_eq!(result, Some(4));
    }
}
