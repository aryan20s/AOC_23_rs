const NUMS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn run(q_input: &str) {
    let mut total_sum = 0u64;

    for line in q_input.split("\n") {
        let mut start_idx = usize::MAX;
        let mut start_num = 0u64;
        let mut end_idx = usize::MIN;
        let mut end_num = 0u64;

        for char_idx in line.chars().zip(0..line.chars().count()) {
            for num_name_idx in NUMS.iter().zip(0..10) {
                let num_name_len = num_name_idx.0.chars().count();
                if num_name_len + char_idx.1 >= line.chars().count() {
                    continue;
                }

                let line_substr_check = line
                    .chars()
                    .skip(char_idx.1)
                    .take(num_name_len)
                    .collect::<String>();

                if *num_name_idx.0 == line_substr_check {
                    if char_idx.1 >= end_idx {
                        end_idx = char_idx.1;
                        end_num = num_name_idx.1 as u64;
                    }
                    if char_idx.1 <= start_idx {
                        start_idx = char_idx.1;
                        start_num = num_name_idx.1 as u64;
                    }
                }
            }

            if char_idx.0.is_numeric() {
                if char_idx.1 >= end_idx {
                    end_idx = char_idx.1;
                    end_num = char_idx.0.to_digit(10).unwrap() as u64;
                }
                if char_idx.1 <= start_idx {
                    start_idx = char_idx.1;
                    start_num = char_idx.0.to_digit(10).unwrap() as u64;
                }
            }
        }

        total_sum += (start_num * 10) + end_num;
    }

    println!("{}", total_sum);
}
