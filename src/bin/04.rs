use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(winning, inventory)| {
            let winning = winning.split_once(":").unwrap().1.trim();
            let inventory = inventory.trim();

            let winning = winning
                .split(" ")
                .filter(|d| !d.is_empty())
                .map(|d| d.trim().parse::<u32>().unwrap());
            let inventory = inventory
                .split(" ")
                .filter(|d| !d.is_empty())
                .map(|d| d.trim().parse::<u32>().unwrap());

            (winning.collect::<Vec<_>>(), inventory)
        })
        .map(|(winning, inventory)| {
            inventory
                .filter(|i| winning.contains(i))
                .fold(0u32, |acc, _| match acc {
                    0 => 1,
                    _ => acc * 2,
                })
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input
        .lines()
        .enumerate()
        .map(|(line_number, line)| (line_number, line.split_once("|").unwrap()))
        .map(|(line_number, (winning, inventory))| {
            let winning = winning.split_once(":").unwrap().1.trim();
            let inventory = inventory.trim();

            let winning = winning
                .split(" ")
                .filter(|d| !d.is_empty())
                .map(|d| d.trim().parse::<u32>().unwrap());
            let inventory = inventory
                .split(" ")
                .filter(|d| !d.is_empty())
                .map(|d| d.trim().parse::<u32>().unwrap());

            (
                (line_number + 1) as u32,
                (winning.collect::<Vec<_>>(), inventory.collect::<Vec<_>>()),
            )
        })
        .collect::<HashMap<_, _>>();

    fn read_card(cards: &HashMap<u32, (Vec<u32>, Vec<u32>)>, card_id: u32) -> u32 {
        if !cards.contains_key(&card_id) {
            return 0;
        }

        let card = cards.get(&card_id).unwrap();
        let matching_numbers = card.1.iter().filter(|d| card.0.contains(d)).count() as u32;

        let mut count = 1;
        if matching_numbers == 0 {
            return count;
        }

        for next_card in (card_id + 1)..(card_id + matching_numbers + 1) {
            count += read_card(cards, next_card);
        }

        count
    }

    Some(
        cards
            .iter()
            .fold(0, |acc, card| acc + read_card(&cards, *card.0)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
