use std::ops::Range;

pub fn parse_seeds(line: &str) -> Result<Vec<u64>, &str> {
    line.split(":")
        .last()
        .ok_or("Invalid seed line: {line}")?
        .trim()
        .split(" ")
        .map(|seed| seed.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_err| "Could not parse seed values: {line}")
}

pub fn parse_mapper(
    lines: Vec<&str>,
) -> Result<Option<(&str, Vec<(Range<u64>, Range<u64>)>)>, &str> {
    let mut lines_iter = lines.into_iter();

    let mapper_name = match lines_iter.next() {
        Some(l) => l,
        None => return Ok(None),
    };

    let maps: Vec<(Range<u64>, Range<u64>)> = lines_iter
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<u64>())
                .collect::<Result<Vec<u64>, _>>()
                .map_err(|_err| "Could not parse mapper line {line} for {mapper_cat}")
        })
        .collect::<Result<Vec<Vec<u64>>, &str>>()?
        .into_iter()
        .map(|mapper_params| match mapper_params.len() {
            3 => Ok((
                Range {
                    start: mapper_params[1],
                    end: mapper_params[1] + mapper_params[2],
                },
                Range {
                    start: mapper_params[0],
                    end: mapper_params[0] + mapper_params[2],
                },
            )),
            _ => Err("Could not parse mapper for {mapper_cat}, invalid line."),
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Some((mapper_name, maps)))
}
