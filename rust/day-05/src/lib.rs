use std::u64::MAX;

pub mod parser;

#[derive(Debug)]
pub struct Mapper {
    trg_start: u64,
    src_start: u64,
    len: u64,
}

impl Mapper {
    pub fn new(trg_start: u64, src_start: u64, len: u64) -> Mapper {
        Mapper {
            trg_start,
            src_start,
            len,
        }
    }

    fn map_value(&self, value: u64) -> Option<u64> {
        if self.src_start <= value && value <= self.src_start + self.len {
            return Some(self.trg_start + value - self.src_start);
        }
        None
    }
}

#[derive(Debug)]
pub struct Almanac {
    seed_ranges: Vec<(u64, u64)>,
    maps_seed_to_soil: Vec<Mapper>,
    maps_soil_to_fertilizer: Vec<Mapper>,
    maps_fertilizer_to_water: Vec<Mapper>,
    maps_water_to_light: Vec<Mapper>,
    maps_light_to_temperature: Vec<Mapper>,
    maps_temperature_to_humidity: Vec<Mapper>,
    maps_humidity_to_location: Vec<Mapper>,
}

impl Almanac {
    pub fn new(seed_ranges: Vec<(u64, u64)>) -> Almanac {
        Almanac {
            seed_ranges,
            maps_seed_to_soil: vec![],
            maps_soil_to_fertilizer: vec![],
            maps_fertilizer_to_water: vec![],
            maps_water_to_light: vec![],
            maps_light_to_temperature: vec![],
            maps_temperature_to_humidity: vec![],
            maps_humidity_to_location: vec![],
        }
    }

    pub fn set_mappers(&mut self, key: &str, mappers: Vec<Mapper>) {
        match key {
            "seed-to-soil map:" => self.maps_seed_to_soil = mappers,
            "soil-to-fertilizer map:" => self.maps_soil_to_fertilizer = mappers,
            "fertilizer-to-water map:" => self.maps_fertilizer_to_water = mappers,
            "water-to-light map:" => self.maps_water_to_light = mappers,
            "light-to-temperature map:" => self.maps_light_to_temperature = mappers,
            "temperature-to-humidity map:" => self.maps_temperature_to_humidity = mappers,
            "humidity-to-location map:" => self.maps_humidity_to_location = mappers,
            _ => (),
        }
    }

    pub fn map_seed(&self, seed: u64) -> (u64, u64) {
        let mut mapped = (seed, seed);
        for mapper_group in [
            &self.maps_seed_to_soil,
            &self.maps_soil_to_fertilizer,
            &self.maps_fertilizer_to_water,
            &self.maps_water_to_light,
            &self.maps_light_to_temperature,
            &self.maps_temperature_to_humidity,
            &self.maps_humidity_to_location,
        ]
        .iter()
        {
            for mapper in *mapper_group {
                match mapper.map_value(mapped.1) {
                    Some(v) => {
                        mapped.1 = v;
                        break;
                    }
                    _ => continue,
                }
            }
        }
        mapped
    }

    pub fn closest_seed(&self) -> (u64, u64) {
        let mut min: (u64, u64) = (0, MAX);
        for seed in self
            .seed_ranges
            .iter()
            .map(|seed_range| (seed_range.0..seed_range.1))
            .flatten()
        {
            let mapped = self.map_seed(seed);
            if mapped.1 < min.1 {
                min = mapped;
            }
        }
        min
    }
}

