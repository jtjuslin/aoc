use std::ops::Range;

use day_05::{
    parser::{parse_mapper, parse_seeds},
    Almanac, Mapper,
};

fn main() {
    let input = include_str!("../input1.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

fn parse(input: &str) -> Result<Almanac, String> {
    let mut lines = input.lines();
    let mut almanac = Almanac::new(
        parse_seeds(
            lines
                .by_ref()
                .take_while(|line| !line.is_empty())
                .collect::<Vec<&str>>()
                .into_iter()
                .next()
                .ok_or("Cannot parse almanac: input is empty")?,
        )?
        .into_iter()
        .map(|seed| Range {
            start: seed,
            end: seed + 1,
        })
        .collect(),
    );

    let mut order: u8 = 0;
    loop {
        match parse_mapper(lines.by_ref().take_while(|line| !line.is_empty()).collect())? {
            Some((mapper_name, maps)) => {
                almanac.add_mapper(Mapper::new(mapper_name.to_string(), order, maps));
                order += 1;
        },
            None => break,
        };
    }

    Ok(almanac)
}

fn process(input: &str) -> Result<u64, String> {
    let almanac = parse(input)?;
    almanac.closest_loc()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let output = process(input);
        assert_eq!(output.unwrap(), 35);
    }
}
