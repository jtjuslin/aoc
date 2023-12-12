use super::Mapper;

pub fn parse_seeds(lines: Vec<&str>) -> Result<Vec<u64>, String> {
    Ok(lines
        .iter()
        .next()
        .ok_or("Input contains no line")?
        .split(":")
        .last()
        .ok_or("Invalid seed line")?
        .trim()
        .split(" ")
        .map(|seed| seed.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()
        .or(Err("Could not parse seed values"))?)
}

pub fn parse_mapper(lines: Vec<&str>) -> Result<Option<(&str, Vec<Mapper>)>, String> {
    let mut lines_iter = lines.into_iter();
    let mappers_cat = match lines_iter.next() {
        Some(line) => line,
        None => return Ok(None),
    };
    let mappers: Vec<Mapper> = lines_iter
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<u64>())
                .collect::<Result<Vec<u64>, _>>()
        })
        .map(|parsed| match parsed {
            Ok(parsed) if parsed.len() == 3 => Ok(Mapper::new(parsed[0], parsed[1], parsed[2])),
            _ => Err("Could not parse mapper, invalid values".to_string()),
        })
        .collect::<Result<Vec<Mapper>, _>>()?;
    Ok(Some((mappers_cat, mappers)))
}
