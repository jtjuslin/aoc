fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

const DIGITS: [(&str, &str); 20] = [
    ("0", "0"),
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("4", "4"),
    ("5", "5"),
    ("6", "6"),
    ("7", "7"),
    ("8", "8"),
    ("9", "9"),
    ("zero", "0"),
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn get_digit(line: &str, first: bool) -> Option<&str> {
    let found = DIGITS.into_iter().filter_map(|(find, digit)| {
        match if first {
            line.find(find)
        } else {
            line.rfind(find)
        } {
            None => None,
            Some(i) => Some((i, digit)),
        }
    });
    match if first {
        found.min_by_key(|d| d.0)
    } else {
        found.max_by_key(|d| d.0)
    } {
        None => None,
        Some(d) => Some(d.1),
    }
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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let output = process(input);
        assert_eq!(281, output);
    }
}
