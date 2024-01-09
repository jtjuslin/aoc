use crate::Card;

pub fn parse_hands(input: &str) -> Result<Vec<([Card; 5], usize)>, &str> {
    input
        .lines()
        .map(
            |line| match line.split_whitespace().collect::<Vec<&str>>() {
                parts if parts.len() == 2 => Ok((parts[0], parts[1])),
                _ => Err("Invalid hand-line."),
            },
        )
        .collect::<Result<Vec<(&str, &str)>, &str>>()?
        .iter()
        .map(|(cards_str, bid_str)| match bid_str.parse::<usize>() {
            Ok(bid) => Ok((*cards_str, bid)),
            Err(_) => Err("Could not parse hand bid."),
        })
        .collect::<Result<Vec<(&str, usize)>, &str>>()?
        .iter()
        .map(
            |(cards_str, bid)| match cards_str.chars().collect::<Vec<char>>() {
                card_labels if card_labels.len() == 5 => Ok((card_labels, *bid)),
                _ => Err("Hand does not have five cards."),
            },
        )
        .collect::<Result<Vec<(Vec<char>, usize)>, &str>>()?
        .iter()
        .map(|(card_labels, bid)| {
            match card_labels
                .iter()
                .map(|label| Card::new(*label).ok_or("Invalid card."))
                .collect::<Result<Vec<Card>, &str>>()
            {
                Ok(cards) => Ok((cards, *bid)),
                Err(err) => Err(err),
            }
        })
        .collect::<Result<Vec<(Vec<Card>, usize)>, &str>>()?
        .iter()
        .map(|(cards, bid)| match cards.len() {
            5 => Ok((
                [
                        cards[0].to_owned(),
                        cards[1].to_owned(),
                        cards[2].to_owned(),
                        cards[3].to_owned(),
                        cards[4].to_owned(),
                ],
                *bid,
            )),
            _ => Err("Hand does not have five cards."),
        })
        .collect::<Result<Vec<([Card; 5], usize)>, &str>>()
}
