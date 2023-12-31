// Advent of Code 2023
// Day 1: Trebuchet?!
// Calibration Values:
//      The sum of the two-digit numbers formed by combining the first and last digit of each line
//      in the document. For example, 1abc2 pqr3stu8vwx a1b2c3d4e5f treb7uchet has a calibration
//      value of 12 + 38 + 15 + 77 = 142.

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|l| {
            let left = l
                .chars()
                .find_map(|c| c.to_digit(10));
            let right = l
                .chars()
                .rev()
                .find_map(|c| c.to_digit(10));

            match (left, right) {
                (Some(l), Some(r)) => Some(l * 10 + r),
                _ => None,
            }
        })
        .sum()
}

const STRING_NUMS: [&str; 18] = ["one", "two", "three", "four", "five", "six", "seven", "eight",
    "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn word_to_u32(word: &str) -> Option<u32> {
    match word {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => word.parse().ok(),
    }
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|l| {
            let left = STRING_NUMS
                .iter()
                .filter_map(|&n| l.match_indices(n).last())
                .min_by_key(|&(x, _)| x)
                .map(|x| word_to_u32(x.1).unwrap());

            let right = STRING_NUMS
                .iter()
                .filter_map(|&n| l.match_indices(n).next())
                .max_by_key(|&(x, _)| x)
                .map(|x| word_to_u32(x.1).unwrap());

            match (left, right) {
                (Some(l), Some(r)) => Some(l * 10 + r),
                _ => None
            }
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_sample() {
        let data = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        println!("{}", data);
        assert_eq!(part1(data), 142);
    }

    #[test]
    fn part2_sample() {
        let data = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        println!("{}", data);
        assert_eq!(part2(data), 281);
    }
}