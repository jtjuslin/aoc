use day_06::parser::parse_races;

fn main() {
    let input = include_str!("../input1.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

fn process(input: &str) -> Result<u64, &str> {
    Ok(parse_races(input)?
        .iter()
        .map(|race| {
            let winners = race.winning_starts();
            winners.end - winners.start
        })
        .product())
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let output = process(input);
        assert_eq!(output.unwrap(), 288);
    }
}
