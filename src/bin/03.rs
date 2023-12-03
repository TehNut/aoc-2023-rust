advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let grid_height = input.lines().count() as u32;
    let grid_width = input.lines().next().unwrap().len() as u32;
    let mut symbol_grid: Vec<bool> = vec![false; (grid_width * grid_height) as usize];
    // ([x], y, part)
    let mut parts: Vec<(Vec<u32>, u32, u32)> = vec![];

    for (line_number, line) in input.lines().enumerate() {
        let mut finding_number = String::new();
        let mut part_locs: Vec<u32> = vec![];

        'inner: for (x, char) in line.char_indices() {
            if !char.is_numeric() && !finding_number.is_empty() {
                parts.push((
                    part_locs.clone(),
                    line_number as u32,
                    finding_number.parse().unwrap(),
                ));
                finding_number.clear();
                part_locs.clear();
            }

            if char == '.' {
                continue 'inner;
            }

            if char.is_numeric() {
                finding_number.push(char);
                part_locs.push(x as u32);
            }

            if !char.is_numeric() {
                for x_loop in -1i32..2 {
                    for y_loop in -1i32..2 {
                        let new_x = x as i32 + x_loop;
                        let new_y = line_number as i32 + y_loop;

                        symbol_grid[(new_y * grid_width as i32 + new_x) as usize] = true;
                    }
                }
            }
        }

        if !finding_number.is_empty() {
            parts.push((
                part_locs.clone(),
                line_number as u32,
                finding_number.parse().unwrap(),
            ));
        }
    }

    let total = parts
        .iter()
        .filter(|(x_locs, y, _part)| {
            x_locs
                .iter()
                .any(|x| symbol_grid[(y * grid_width + x) as usize])
        })
        .map(|(_, _, part)| part)
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    // ([x], y, part)
    let mut parts: Vec<(Vec<u32>, u32, u32)> = vec![];
    // (x, y)
    let mut potential_gears: Vec<(u32, u32)> = vec![];

    for (line_number, line) in input.lines().enumerate() {
        let mut finding_number = String::new();
        let mut part_locs: Vec<u32> = vec![];

        'inner: for (x, char) in line.char_indices() {
            if !char.is_numeric() && !finding_number.is_empty() {
                parts.push((
                    part_locs.clone(),
                    line_number as u32,
                    finding_number.parse().unwrap(),
                ));
                finding_number.clear();
                part_locs.clear();
            }

            if char == '.' {
                continue 'inner;
            }

            if char.is_numeric() {
                finding_number.push(char);
                part_locs.push(x as u32);
            }

            if char == '*' {
                potential_gears.push((x as u32, line_number as u32));
            }
        }

        if !finding_number.is_empty() {
            parts.push((
                part_locs.clone(),
                line_number as u32,
                finding_number.parse().unwrap(),
            ));
        }
    }

    let total: u32 = potential_gears
        .iter()
        .filter_map(|g: &(u32, u32)| {
            let matched_parts = parts
                .iter()
                .filter(|(part_x_locs, part_y, _)| {
                    for x_loop in -1i32..2 {
                        for y_loop in -1i32..2 {
                            let off_x = g.0 as i32 + x_loop;
                            let off_y = g.1 as i32 + y_loop;

                            for part_x in part_x_locs {
                                if part_x.eq(&(off_x as u32)) && part_y.eq(&(off_y as u32)) {
                                    return true;
                                }
                            }
                        }
                    }

                    false
                })
                .map(|p| p.2)
                .collect::<Vec<_>>();

            if matched_parts.len() == 2 {
                Some(matched_parts.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
