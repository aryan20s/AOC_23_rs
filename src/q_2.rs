pub fn run(q_input: &str) {
    let mut game_id_sum = 0u64;
    let mut set_power_sum = 0u64;

    for line in q_input.split("\n") {
        let mut line_sep = line.split(": ");
        if line_sep.clone().count() != 2 {
            //line should have 2 elements after splitting with :
            continue;
        }
        let line_sep_1 = line_sep.next().unwrap();
        let line_sep_2 = line_sep.next().unwrap();

        //technically this game id variable is unneeded but i thought there
        //might be skips to trick code which just used a counter for game id
        let game_id = line_sep_1
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let mut game_id_possible = true;

        let mut max_red = 0u64;
        let mut max_grn = 0u64;
        let mut max_blu = 0u64;

        //per set of cubes in each game
        for cube_set in line_sep_2.split("; ") {
            //per cube color in each set
            for cube_elem in cube_set.split(", ") {
                //number, color (in the format ["2", "red"])
                let mut elem_data = cube_elem.split(" ");
                if elem_data.clone().count() != 2 {
                    //this should not be possible, since splitting
                    //"2 red" etc should always yield 2 elements
                    panic!("???");
                }
                let color_count = elem_data.next().unwrap().parse::<u64>().unwrap();
                let color_name = elem_data.next().unwrap().trim();

                match color_name {
                    "red" => {
                        max_red = max_red.max(color_count);
                        if color_count > 12 {
                            game_id_possible = false;
                        }
                    }
                    "green" => {
                        max_grn = max_grn.max(color_count);
                        if color_count > 13 {
                            game_id_possible = false;
                        }
                    }
                    "blue" => {
                        max_blu = max_blu.max(color_count);
                        if color_count > 14 {
                            game_id_possible = false;
                        }
                    }
                    _ => {
                        //any other color
                        game_id_possible = false;
                    }
                }
            }
        }

        set_power_sum += max_red * max_grn * max_blu;

        if game_id_possible {
            game_id_sum += game_id;
        }
    }

    println!("Game IDs possible sum with 12R, 13G, 14B: {}", game_id_sum);
    println!("Set power sum: {}", set_power_sum);
}
