advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let values = input
        .lines()
        .map(|line| {
            let digits_in_line = line
                .chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_string())
                .collect::<Vec<String>>();

            format!(
                "{}{}",
                digits_in_line.first().unwrap(),
                digits_in_line.last().unwrap()
            )
        })
        .map(|digit| digit.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Some(values.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let string_digits: Vec<&str> = Vec::from([
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]);

    let values = input
        .lines()
        .map(|line| {
            let mut digit_locations: Vec<(String, usize)> = Vec::new();

            // First get the locations of each numeric character
            line.chars()
                .enumerate()
                .filter(|(_, c)| c.is_numeric())
                .for_each(|(idx, c)| digit_locations.push((c.to_owned().to_string(), idx)));

            // Next get the locations of each word digit
            string_digits
                .iter()
                .map(|d| (d, line.match_indices(d)))
                .for_each(|(d, matches)| {
                    matches.for_each(|(pos, _)| {
                        let digit_index = string_digits.iter().position(|e| e.eq(d)).unwrap();
                        digit_locations.push((format!("{}", digit_index + 1), pos))
                    })
                });

            let first_digit = &digit_locations
                .iter()
                .min_by(|(_, pos1), (_, pos2)| pos1.cmp(pos2));
            let last_digit = &digit_locations
                .iter()
                .max_by(|(_, pos1), (_, pos2)| pos1.cmp(pos2));

            format!("{}{}", first_digit.unwrap().0, last_digit.unwrap().0)
        })
        .map(|digit| digit.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Some(values.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let result = part_two("4fourtwo86tkdkxtwo");
        assert_eq!(result, Some(42)) // Wrong as 46
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
