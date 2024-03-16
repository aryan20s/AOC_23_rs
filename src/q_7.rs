use std::any::Any;
use std::cmp::Ordering;

#[derive(PartialEq, Clone, Debug)]
struct Game {
    hand_cards: String,
    bet_amount: u64,
}

#[derive(Clone, Copy, Debug)]
enum HandType {
    Five = 6,
    Four = 5,
    FullHouse = 4,
    Three = 3,
    TwoPair = 2,
    OnePair = 1,
    High = 0,
}

impl Game {
    fn new(hand_cards: &str, bet_amount: u64) -> Self {
        Self {
            hand_cards: String::from(hand_cards),
            bet_amount,
        }
    }

    fn get_hand_type(&self) -> HandType {
        let mut is_five = false;
        let mut is_four = false;
        let mut is_full = false;
        let mut is_three = false;
        let mut is_two_pair = false;
        let mut is_pair = false;

        let mut last_match_char = '\0';

        for card in self.hand_cards.chars() {
            let mut found_count = 0u64;
            for cmp_card in self.hand_cards.chars() {
                if card == cmp_card {
                    if is_pair && (found_count == 1) && (last_match_char != card) {
                        is_two_pair = true;
                    }
                    if is_three && (found_count == 1) && (last_match_char != card) {
                        is_full = true;
                    }
                    if is_pair && (found_count == 2) && (last_match_char != card) {
                        is_full = true;
                    }
                    found_count += 1;
                }
            }

            if found_count != 1 {
                last_match_char = card;
            }

            match found_count {
                5 => is_five = true,
                4 => is_four = true,
                3 => is_three = true,
                2 => is_pair = true,
                _ => {}
            }
        }

        if is_five {
            return HandType::Five;
        } else if is_four {
            return HandType::Four;
        } else if is_full {
            return HandType::FullHouse;
        } else if is_three {
            return HandType::Three;
        } else if is_two_pair {
            return HandType::TwoPair;
        } else if is_pair {
            return HandType::OnePair;
        } else {
            return HandType::High;
        }
    }
}

const CARD_POWER: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let rank_self = self.get_hand_type();
        let rank_other = other.get_hand_type();

        if (rank_self as i32) > (rank_other as i32) {
            return Some(Ordering::Greater);
        } else if (rank_self as i32) < (rank_other as i32) {
            return Some(Ordering::Less);
        }

        for cards_cmp in self.hand_cards.chars().zip(other.hand_cards.chars()) {
            let idx_1 = CARD_POWER.iter().position(|c| *c == cards_cmp.0).unwrap();
            let idx_2 = CARD_POWER.iter().position(|c| *c == cards_cmp.1).unwrap();

            if idx_2 > idx_1 {
                return Some(Ordering::Greater);
            } else if idx_2 < idx_1 {
                return Some(Ordering::Less);
            }
        }

        return Some(Ordering::Equal);
    }
}

pub fn run(q_input: &str) {
    let q_lines = q_input.split("\n");
    let mut games = Vec::<Game>::new();
    for line in q_lines {
        let mut game_data = line.split_ascii_whitespace();
        games.push(Game::new(
            game_data.next().unwrap(),
            game_data.next().unwrap().parse::<u64>().unwrap(),
        ));
    }

    games.sort_by(|g1, g2| g1.partial_cmp(g2).unwrap());
    let winnings = games.iter().enumerate().fold(0u64, |sum, other| {
        sum + (other.0 as u64 + 1) * other.1.bet_amount
    });

    for game in games {
        println!("{} {}", game.hand_cards, game.bet_amount);
    }

    println!("Winnings: {}", winnings);
}
