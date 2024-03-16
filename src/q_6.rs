pub fn run(q_input: &str) {
    let mut ways_to_beat_1 = 1u64;
    let mut ways_to_beat_2 = 0u64;

    let q_lines: Vec<&str> = q_input.split("\n").collect();
    let times: Vec<u64> = q_lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|elem| elem.parse::<u64>().unwrap())
        .collect();

    let distances: Vec<u64> = q_lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .map(|elem| elem.parse::<u64>().unwrap())
        .collect();

    for time_idx in times.iter().enumerate() {
        let mut ways_to_beat = 0u64;
        for wait_len in 1..(*time_idx.1) {
            let time_remaining = *time_idx.1 - wait_len;
            if (wait_len * time_remaining) > distances[time_idx.0] {
                ways_to_beat += 1;
            }
        }
        ways_to_beat_1 *= ways_to_beat;
    }

    let time_concat: u64 = q_lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .fold("".to_string(), |old, new| old + new)
        .parse::<u64>()
        .unwrap();

    let distance_concat: u64 = q_lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .fold("".to_string(), |old, new| old + new)
        .parse::<u64>()
        .unwrap();

    for wait_len in 1..time_concat {
        let time_remaining = time_concat - wait_len;
        if (wait_len * time_remaining) > distance_concat {
            ways_to_beat_2 += 1;
        }
    }

    println!("Part 1: {}", ways_to_beat_1);
    println!("Part 2: {}", ways_to_beat_2);
}
