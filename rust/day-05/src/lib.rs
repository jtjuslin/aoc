use itertools::Itertools;
use std::{
    cmp::{max, min},
    ops::Range,
};

pub mod parser;

#[derive(Debug)]
pub struct Mapper {
    name: String,
    order: u8,
    maps: Vec<(Range<u64>, Range<u64>)>,
}

impl Mapper {
    pub fn new(name: String, order: u8, maps: Vec<(Range<u64>, Range<u64>)>) -> Mapper {
        Mapper { name, order, maps }
    }

    fn map(&self, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut mapped: Vec<Range<u64>> = vec![];
        let mut unmapped: Vec<Range<u64>> = ranges;
        for (source, target) in self.maps.clone() {
            unmapped = unmapped
                .into_iter()
                .map(|mut range| {
                    let out_left = Range {
                        start: range.start,
                        end: min(range.end, source.start),
                    };
                    if !out_left.is_empty() {
                        range.start = out_left.end;
                    }
                    let out_right = Range {
                        start: max(range.start, source.end),
                        end: range.end,
                    };
                    if !out_right.is_empty() {
                        range.end = out_right.start;
                    }
                    if !range.is_empty() {
                        mapped.push(Range {
                            start: target.start + (range.start - source.start),
                            end: target.end - (source.end - range.end),
                        });
                    }
                    return vec![out_left, out_right];
                })
                .flatten()
                .filter(|range| !range.is_empty())
                .collect();
        }
        mapped.extend(unmapped);
        return mapped;
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<Range<u64>>,
    mappers: Vec<Mapper>,
}

impl Almanac {
    pub fn new(seeds: Vec<Range<u64>>) -> Almanac {
        Almanac {
            seeds,
            mappers: vec![],
        }
    }

    pub fn add_mapper(&mut self, mapper: Mapper) {
        self.mappers.push(mapper);
    }

    pub fn closest_loc(&self) -> Result<u64, String> {
        match self
            .mappers
            .iter()
            .sorted_by_key(|mapper| mapper.order)
            .fold(self.seeds.clone(), |unmapped, mapper| mapper.map(unmapped))
            .iter()
            .sorted_by_key(|mapped| mapped.start)
            .next()
        {
            Some(range) => Ok(range.start),
            _ => Err("No seed was mapped.".to_string()),
        }
    }
}
