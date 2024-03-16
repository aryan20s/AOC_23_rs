use std::collections::HashMap;

#[derive(PartialEq)]
enum State {
    ONCE,
    TWICE,
    MORE,
}

pub fn run(q_input: &str) {
    let mut data_array = Vec::<char>::new();
    let mut line_elem_count = 0;
    let mut part_id_sum = 0u64;
    let mut gear_ratios_sum = 0u64;
    let mut gear_idxs = HashMap::<usize, (u64, State)>::new();

    for line in q_input.split("\n") {
        if line_elem_count == 0 {
            line_elem_count = line.chars().count();
        }

        for char in line.chars() {
            data_array.push(char);
        }
    }

    let mut idx = 0usize;
    while idx < data_array.len() {
        let char = data_array[idx];

        if char.is_numeric() {
            let mut check_idx = idx;
            let mut num = 0u64;
            let mut test_char = data_array[check_idx];
            let mut adj_symb = false;
            let mut gear_idx = usize::MAX;

            while (check_idx < data_array.len()) && (test_char.is_numeric()) {
                if !adj_symb {
                    let mut positions = [
                        (0usize, None),
                        (0usize, None),
                        (0usize, None),
                        (0usize, None),
                        (0usize, None),
                        (0usize, None),
                        (0usize, None),
                        (0usize, None),
                    ];
                    let on_left_side = (check_idx % line_elem_count) == 0;
                    let on_right_side = (check_idx % line_elem_count) == (line_elem_count - 1);

                    if check_idx > line_elem_count {
                        if (check_idx > line_elem_count + 1) && !on_left_side {
                            positions[0] = (
                                check_idx - line_elem_count - 1,
                                data_array.get(check_idx - line_elem_count - 1),
                            );
                        }
                        positions[1] = (
                            check_idx - line_elem_count,
                            data_array.get(check_idx - line_elem_count),
                        );
                    }

                    if (check_idx >= line_elem_count) && !on_right_side {
                        positions[2] = (
                            check_idx + 1 - line_elem_count,
                            data_array.get(check_idx + 1 - line_elem_count),
                        );
                    }

                    if (check_idx > 0) && !on_left_side {
                        positions[3] = (check_idx - 1, data_array.get(check_idx - 1));
                    }

                    if !on_right_side {
                        positions[4] = (check_idx + 1, data_array.get(check_idx + 1));
                    }

                    if !on_left_side {
                        positions[5] = (
                            check_idx + line_elem_count - 1,
                            data_array.get(check_idx + line_elem_count - 1),
                        );
                    }

                    positions[6] = (
                        check_idx + line_elem_count,
                        data_array.get(check_idx + line_elem_count),
                    );

                    if !on_right_side {
                        positions[7] = (
                            check_idx + line_elem_count + 1,
                            data_array.get(check_idx + line_elem_count + 1),
                        );
                    }

                    for pos_check in positions {
                        match pos_check.1 {
                            None => {}
                            Some(ch) => {
                                if (!ch.is_numeric()) && (*ch != '.') {
                                    adj_symb = true;

                                    if *ch == '*' {
                                        gear_idx = pos_check.0;
                                    }

                                    break;
                                }
                            }
                        }
                    }
                }

                num = num * 10 + test_char.to_digit(10).unwrap() as u64;
                check_idx += 1;
                match data_array.get(check_idx) {
                    None => {}
                    Some(ch) => {
                        test_char = *ch;
                    }
                }
            }
            if adj_symb {
                part_id_sum += num;
                if gear_idx != usize::MAX {
                    if gear_idxs.contains_key(&gear_idx) {
                        let val = gear_idxs.get_mut(&gear_idx).unwrap();
                        if val.1 == State::TWICE {
                            val.1 = State::MORE;
                        }
                        if val.1 == State::ONCE {
                            val.1 = State::TWICE;
                            val.0 *= num;
                        }
                    } else {
                        gear_idxs.insert(gear_idx, (num, State::ONCE));
                    }
                }
            }
            idx += check_idx - idx - 1;
        }

        idx += 1;
    }

    for gear in gear_idxs {
        if gear.1 .1 == State::TWICE {
            gear_ratios_sum += gear.1 .0;
        }
    }

    println!("Part ID Sum: {}", part_id_sum);
    println!("Gear Ratios Sum: {}", gear_ratios_sum);
}
