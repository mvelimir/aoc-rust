advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first_vec, mut second_vec): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|x| {
            let mut number_it = x.split_whitespace().map(|x| x.parse::<u32>().unwrap());

            (number_it.next().unwrap(), number_it.next().unwrap())
        })
        .unzip();

    first_vec.sort();
    second_vec.sort();

    let sum: u32 = first_vec
        .iter()
        .zip(second_vec.iter())
        .map(|x| x.0.abs_diff(*x.1))
        .sum();

    Some(sum)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
