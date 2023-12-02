advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let totals = input
        .lines()
        .map(|line| {
            let (game_id, game) = line.split_once(": ").unwrap();
            let game_id = game_id.split_once(" ").unwrap().1.parse::<u32>().unwrap();

            // Get all the sets from the game (red, green, blue)
            let mut round_sets: Vec<CubeSet> = Vec::new();
            for round in game.trim().split(";") {
                let sets = round.trim().split(",");
                let mut values: CubeSet = (0, 0, 0);

                sets.for_each(|set| {
                    let split = set.trim().split_once(" ").unwrap();
                    match split.1.trim() {
                        "red" => values.0 = split.0.trim().parse().unwrap(),
                        "green" => values.1 = split.0.trim().parse().unwrap(),
                        "blue" => values.2 = split.0.trim().parse().unwrap(),
                        _ => {}
                    };
                });

                round_sets.push(values);
            }

            (game_id, round_sets)
        })
        .filter(|(_game_id, sets)| {
            // Filter out anything not matching the requirements
            let flag = sets.iter().fold(true, |acc, set| {
                if !acc {
                    return false;
                };
                set.0 <= 12 && set.1 <= 13 && set.2 <= 14
            });

            flag
        })
        .map(|(game_id, _)| game_id)
        .sum();

    Some(totals)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total = input
        .lines()
        .map(|line| {
            let line = line.split_once(": ").unwrap().1;

            let mut max_cubes: CubeSet = (0, 0, 0);
            line.split(";").for_each(|round| {
                let sets = round.trim().split(",");
                sets.for_each(|set| {
                    let (count, color) = set.trim().split_once(" ").unwrap();

                    let count = count.trim().parse::<u32>().unwrap();
                    match color.trim() {
                        "red" => {
                            if count > max_cubes.0 {
                                max_cubes.0 = count;
                            }
                        }
                        "green" => {
                            if count > max_cubes.1 {
                                max_cubes.1 = count;
                            }
                        }
                        "blue" => {
                            if count > max_cubes.2 {
                                max_cubes.2 = count;
                            }
                        }
                        _ => {}
                    };
                });
            });

            max_cubes.0 * max_cubes.1 * max_cubes.2
        })
        .sum();

    Some(total)
}

/** red, green, blue */
type CubeSet = (u32, u32, u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
