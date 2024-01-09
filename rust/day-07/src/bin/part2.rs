use day_07::{parser::parse_hands, Camel, Card};

fn main() {
    let input = include_str!("../input1.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

fn process(input: &str) -> Result<usize, &str> {
    let hands = parse_hands(input)?;
    let mut camel = Camel::new(hands, Some(Card::J))?;
    Ok(camel.play())
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let output = process(input);
        assert_eq!(output.unwrap(), 5905);
    }
}
