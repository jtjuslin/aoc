use day_06::parser::parse_races_2;

fn main() {
    let input = include_str!("../input2.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

fn process(input: &str) -> Result<u64, &str> {
    let winners = parse_races_2(input)?.winning_starts();
    Ok(winners.end - winners.start)
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let output = process(input);
        assert_eq!(output.unwrap(), 71503);
    }
}
