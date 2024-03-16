use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone)]
enum CurMode {
    Starting = 0,
    Seed2Soil = 1,
    Soil2Fert = 2,
    Fert2Water = 3,
    Water2Light = 4,
    Light2Temp = 5,
    Temp2Humid = 6,
    Humid2Loc = 7,
}

struct MapData {
    source_start: u64,
    dest_start: u64,
    map_count: u64,
}

impl MapData {
    fn new(source_start: u64, dest_start: u64, map_count: u64) -> Self {
        Self {
            source_start,
            dest_start,
            map_count,
        }
    }

    fn get_mapped_output(&self, input: u64) -> Option<u64> {
        if input >= self.source_start {
            let zero_off_in = input - self.source_start;
            if zero_off_in < self.map_count {
                return Some(zero_off_in + self.dest_start);
            }
        }

        None
    }
}

pub fn run(q_input: &str) {
    let mut seeds = vec![0u64; 0];
    let mut seed_to_soil = Vec::<MapData>::new();
    let mut soil_to_fert = Vec::<MapData>::new();
    let mut fert_to_water = Vec::<MapData>::new();
    let mut water_to_light = Vec::<MapData>::new();
    let mut light_to_temp = Vec::<MapData>::new();
    let mut temp_to_humid = Vec::<MapData>::new();
    let mut humid_to_loc = Vec::<MapData>::new();
    let mut cur_mode = CurMode::Starting;

    for line in q_input.split("\n") {
        if line.trim() == "" {
            continue;
        }

        if line.starts_with("seeds: ") {
            seeds = line
                .split("seeds: ")
                .nth(1)
                .unwrap()
                .split_ascii_whitespace()
                .map(|elem| elem.parse::<u64>().unwrap())
                .collect();
            continue;
        }

        let mut should_continue = true;
        match line.trim() {
            "seed-to-soil map:" => cur_mode = CurMode::Seed2Soil,
            "soil-to-fertilizer map:" => cur_mode = CurMode::Soil2Fert,
            "fertilizer-to-water map:" => cur_mode = CurMode::Fert2Water,
            "water-to-light map:" => cur_mode = CurMode::Water2Light,
            "light-to-temperature map:" => cur_mode = CurMode::Light2Temp,
            "temperature-to-humidity map:" => cur_mode = CurMode::Temp2Humid,
            "humidity-to-location map:" => cur_mode = CurMode::Humid2Loc,
            _ => should_continue = false,
        }
        if should_continue {
            continue;
        }

        let range_spl: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|elem| elem.parse::<u64>().unwrap())
            .collect();
        let dest_start = range_spl[0];
        let source_start = range_spl[1];
        let map_count = range_spl[2];
        let mut map_to_insert;
        match cur_mode {
            CurMode::Starting => panic!(),
            CurMode::Seed2Soil => map_to_insert = &mut seed_to_soil,
            CurMode::Soil2Fert => map_to_insert = &mut soil_to_fert,
            CurMode::Fert2Water => map_to_insert = &mut fert_to_water,
            CurMode::Water2Light => map_to_insert = &mut water_to_light,
            CurMode::Light2Temp => map_to_insert = &mut light_to_temp,
            CurMode::Temp2Humid => map_to_insert = &mut temp_to_humid,
            CurMode::Humid2Loc => map_to_insert = &mut humid_to_loc,
        }

        map_to_insert.push(MapData::new(source_start, dest_start, map_count));
    }

    let mut lowest_loc = u64::MAX;
    let mut total_iters = 0u64;
    let mut total_time = Duration::from_secs(0);
    for seed_elem in seeds.iter().enumerate().step_by(2) {
        println!(
            "Running {} times for ({}, {})",
            seeds[seed_elem.0 + 1],
            seed_elem.0,
            seed_elem.1
        );
        total_iters += seeds[seed_elem.0 + 1];
        let before = Instant::now();

        for seed in *seed_elem.1..(seeds[seed_elem.0 + 1] + *seed_elem.1) {
            let mut soil_map = None;
            let mut soil = 0;
            for seed_soil in &seed_to_soil {
                let temp = seed_soil.get_mapped_output(seed);
                if temp.is_some() {
                    soil_map = temp;
                    break;
                }
            }
            match soil_map {
                None => soil = seed,
                Some(x) => soil = x,
            }

            let mut fert_map = None;
            let mut fert = 0;
            for soil_fert in &soil_to_fert {
                let temp = soil_fert.get_mapped_output(soil);
                if temp.is_some() {
                    fert_map = temp;
                    break;
                }
            }
            match fert_map {
                None => fert = soil,
                Some(x) => fert = x,
            }

            let mut water_map = None;
            let mut water = 0;
            for fert_water in &fert_to_water {
                let temp = fert_water.get_mapped_output(fert);
                if temp.is_some() {
                    water_map = temp;
                    break;
                }
            }
            match water_map {
                None => water = fert,
                Some(x) => water = x,
            }

            let mut light_map = None;
            let mut light = 0;
            for water_light in &water_to_light {
                let temp = water_light.get_mapped_output(water);
                if temp.is_some() {
                    light_map = temp;
                    break;
                }
            }
            match light_map {
                None => light = water,
                Some(x) => light = x,
            }

            let mut temp_map = None;
            let mut temp = 0;
            for light_temp in &light_to_temp {
                let temp = light_temp.get_mapped_output(light);
                if temp.is_some() {
                    temp_map = temp;
                    break;
                }
            }
            match temp_map {
                None => temp = light,
                Some(x) => temp = x,
            }

            let mut humid_map = None;
            let mut humid = 0;
            for temp_humid in &temp_to_humid {
                let temp = temp_humid.get_mapped_output(temp);
                if temp.is_some() {
                    humid_map = temp;
                    break;
                }
            }
            match humid_map {
                None => humid = temp,
                Some(x) => humid = x,
            }

            let mut loc_map = None;
            let mut loc = 0;
            for humid_loc in &humid_to_loc {
                let temp = humid_loc.get_mapped_output(humid);
                if temp.is_some() {
                    loc_map = temp;
                    break;
                }
            }
            match loc_map {
                None => loc = humid,
                Some(x) => loc = x,
            }

            lowest_loc = lowest_loc.min(loc);
        }

        total_time += before.elapsed();
    }
    println!("Lowest loc: {}", lowest_loc);
    println!(
        "Took {}.{}s for {} iters.",
        total_time.as_secs(),
        total_time.as_millis() % 1000,
        total_iters
    );
}
