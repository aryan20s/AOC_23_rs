use std::collections::HashMap;

pub fn run(q_input: &str) {
    let mut total_score = 0u64;
    let mut total_cards_gotten = 0u64;
    let mut card_extra_count = HashMap::<u64, u64>::new();

    for line in q_input.split("\n") {
        let mut line_sep = line.split(": ");
        if line_sep.clone().count() != 2 {
            println!("Invalid line sep (:) count: {}", line);
            continue;
        }
        let card_no_split = line_sep.next().unwrap().split(" ");
        let mut card_no = 0;
        for card_no_it in card_no_split {
            match card_no_it.parse::<u64>() {
                Err(_) => {}
                Ok(val) => {
                    card_no = val;
                }
            }
        }
        let cards_gotten = line_sep.next().unwrap();

        let mut win_have_cards = cards_gotten.split(" | ");
        if win_have_cards.clone().count() != 2 {
            continue;
        }
        let win_cards = win_have_cards.next().unwrap();
        let have_cards = win_have_cards.next().unwrap();

        let mut win_cards_num = vec![0u64; 0];
        let mut have_cards_num = vec![0u64; 0];
        for win_card in win_cards.split(" ") {
            if win_card == "" {
                continue;
            }
            win_cards_num.push(win_card.parse::<u64>().unwrap());
        }
        for have_card in have_cards.split(" ") {
            if have_card == "" {
                continue;
            }
            have_cards_num.push(have_card.parse::<u64>().unwrap());
        }

        let mut win_count = 0u64;
        for win_card_num in win_cards_num {
            if have_cards_num.contains(&win_card_num) {
                win_count += 1;
            }
        }

        let my_count = *card_extra_count.get(&card_no).unwrap_or(&0) + 1;
        for card_offset in 1..=win_count {
            let card_idx = card_no + card_offset;
            if card_extra_count.contains_key(&card_idx) {
                *card_extra_count.get_mut(&card_idx).unwrap() += my_count;
            } else {
                card_extra_count.insert(card_idx, my_count);
            }
        }

        if win_count != 0 {
            total_score += 1u64.wrapping_shl((win_count - 1) as u32);
        }

        total_cards_gotten += 1;
    }

    for card_iter in card_extra_count {
        total_cards_gotten += card_iter.1;
    }

    println!("Total score: {}", total_score);
    println!("Total cards gotten: {}", total_cards_gotten);
}
