use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

#[derive(Debug)]
struct Card {
   number: u32,
   matching_numbers: Vec<u32>,
}

impl Card {
    fn worth(&self) -> u32 {
        if self.matching_numbers.len() == 0 {
            return 0;
        }
        let two: u32 = 2;
        two.pow(self.matching_numbers.len() as u32 - 1)
    }    
}

fn parse(input: &str) -> Result<Vec<Card>, String> {
    input.lines().map(|line| {
        let err = Err(format!("Invalid game: {:?}", line));
        let (game_info, numbers) = match line.split(":").map(|s| s.trim()).collect::<Vec<&str>>() {
            parts if parts.len() == 2 => (parts[0], parts[1]),
            _ => return err,
        };
        let game_number = match game_info.split_whitespace().collect::<Vec<&str>>() {
            game_data if game_data.len() == 2 => match game_data[1].parse::<u32>() {
                Ok(game_number) => game_number,
                Err(_) => return err,
            },
            _ => return err,
        };
        let (winning_numbers, lucky_numbers): (Vec<u32>, Vec<u32>) = match numbers.split("|").collect::<Vec<&str>>() {
            numbers if numbers.len() == 2 => {
                (match numbers[0].split_whitespace().map(|number| number.trim().parse::<u32>()).collect() { Err(_) => return err, Ok(c) => c },
                match numbers[1].split_whitespace().map(|number| number.trim().parse::<u32>()).collect() { Err(_) => return err, Ok(c) => c})

            },
            _ => return err,
        };
        let matching_numbers = winning_numbers.into_iter().filter(|number| lucky_numbers.contains(number)).collect();
        Ok(Card { number: game_number, matching_numbers })
    }).collect()
}

fn process(input: &str) -> Result<u32, String> {
    let cards_map = parse(input)?.into_iter().map(|card| {
        (card.number, card)
    }).collect::<BTreeMap<_, _>>();
    let mut card_amounts = cards_map.iter().map(|(key, _)| (*key, 1)).collect::<BTreeMap<u32, u32>>();
    for (key, card) in cards_map {
        let lower = key + 1;
        let upper = lower + card.matching_numbers.len() as u32;
        let repeat = match card_amounts.get(&key) {
            Some(amount) => amount,
            _ => continue,
        };
        for _ in 0..*repeat {
            for game_number in lower..upper {
                if let Some(amount) = card_amounts.get(&game_number) {
                    card_amounts.insert(game_number, amount + 1);
                }
            }
        }
    }
    Ok(card_amounts.values().sum())
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = process(input);
        assert_eq!(output.unwrap(), 30);
    }
}
