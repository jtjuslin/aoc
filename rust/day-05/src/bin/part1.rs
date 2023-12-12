use day_05::{Almanac, parser::{parse_seeds, parse_mapper}};

fn main() {
    let input = include_str!("../input1.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

fn parse(input: &str) -> Result<Almanac, String> {
    let mut lines = input.lines();
    let mut almanac = match parse_seeds(
        lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<&str>>(),
    ) {
        Ok(seeds) => Almanac::new(seeds.iter().map(|seed| (*seed, *seed + 1)).collect()),
        _ => return Err("Invalid seeds".to_string()),
    };
    loop {
        match parse_mapper(lines.by_ref().take_while(|line| !line.is_empty()).collect()) {
            Ok(data) => match data {
                Some((mapper_cat, mappers)) => almanac.set_mappers(mapper_cat, mappers),
                None => break,
            },
            Err(err) => return Err(err),
        };
    }
    Ok(almanac)
}

fn process(input: &str) -> Result<u64, String> {
    let almanac = parse(input)?;
    Ok(almanac.closest_seed().1)
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
