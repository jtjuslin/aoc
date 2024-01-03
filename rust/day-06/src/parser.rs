use std::iter::zip;

use super::Race;

// Format:
// Time:      7  15   30
// Distance:  9  40  200
pub fn parse_races(input: &str) -> Result<Vec<Race>, &str> {
    let mut lines = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|val| val.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()
    });

    let times = lines
        .next()
        .ok_or("Invalid input.")?
        .or(Err("Could not parse times"))?;
    let dists = lines
        .next()
        .ok_or("Invalid input.")?
        .or(Err("Could not parse times"))?;

    Ok(zip(times, dists)
        .map(|(time, dist)| Race::new(time, dist))
        .collect::<Vec<Race>>())
}

// Format:
// Time:      7  15   30
// Distance:  9  40  200
//
// Twist: ignore the whitespace so there's only one race like this 
// Time:      71520
// Distance:  940200
pub fn parse_races_2(input: &str) -> Result<Race, &str> {
    match input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .fold(String::new(), |a, b| a + b)
                .parse::<u64>()
        })
        .collect::<Result<Vec<u64>, _>>()
    {
        Ok(vals) if vals.len() == 2 => Ok(Race::new(vals[0], vals[1])),
        _ => Err("Invalid input."),
    }
}
