use itertools::Either;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn get_digit(line: &str, first: bool) -> Option<char> {
    return if first {
        Either::Left(line.chars())
    } else {
        Either::Right(line.chars().rev())
    }
    .skip_while(|&c| !c.is_digit(10))
    .next();
}

fn process(input: &str) -> u32 {
    return input
        .lines()
        .filter_map(|line| {
            let first = match get_digit(line, true) {
                None => return None,
                Some(c) => c,
            };
            let last = match get_digit(line, false) {
                None => return None,
                Some(c) => c,
            };
            format!("{first}{last}").parse::<u32>().ok()
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let output = process(input);
        assert_eq!(142, output);
    }
}
