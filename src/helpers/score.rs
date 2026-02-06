use crate::enums::FinalScore;
use crate::models::Card;

pub fn calculate_score(hand: &Vec<Card>) -> u32 {
    let max_score = hand.iter().map(|card| card.value).sum::<u32>();
    match (max_score > 21, nr_of_aces_in_hand(hand)) {
        (false, _) => max_score,
        (true, x) => replace_ace_values(max_score, x)
    }
}

fn replace_ace_values(score: u32, remaining_aces: u32) -> u32 {
    if remaining_aces < 1 || score < 21 {
        return score
    }
    replace_ace_values(score - 10, remaining_aces - 1)
}

fn nr_of_aces_in_hand(hand: &Vec<Card>) -> u32 {
    hand.iter().filter(|card| card.value == 11).count() as u32
}

pub fn compare_scores(player_score: u32, dealer_score: u32) -> FinalScore {
    match (player_score, dealer_score) {
        (x, _) if x > 21 => FinalScore::DealerWins,
        (x, y) if x <= 21 && y > 21 => FinalScore::UserWins,
        (x, y) if x <= 21 && x > y => FinalScore::UserWins,
        (x, y) if x == y => FinalScore::Draw,
        (x, y) if x < y => FinalScore::DealerWins,
        _ => {
            println!("Unexpected case!");
            FinalScore::Draw
        }
    }
}

pub fn check_final_score(player_hand: &Vec<Card>, dealer_hand: &Vec<Card>) -> FinalScore {
    let player_score = calculate_score(player_hand);
    let dealer_score = calculate_score(dealer_hand);
    compare_scores(player_score, dealer_score)
}

#[cfg(test)]
mod tests {
    use ansi_term::Color::{Black, Red};
    use yare::parameterized;
    use super::*;

    #[parameterized(
    ka = {
        vec![
            Card{representation: String::from("♣K"), output_style: Black.bold(), value: 10},
            Card{representation: String::from("♥A"), output_style: Red.bold(), value: 11}
        ], 21
    },
    kaa = {
        vec![
            Card{representation: String::from("♣K"), output_style: Black.bold(), value: 10},
            Card{representation: String::from("♥A"), output_style: Red.bold(), value: 11},
            Card{representation: String::from("♠A"), output_style: Black.bold(), value: 11}
        ], 12
    },
    qka = {
        vec![
            Card{representation: String::from("♣K"), output_style: Black.bold(), value: 10},
            Card{representation: String::from("♥A"), output_style: Red.bold(), value: 11},
            Card{representation: String::from("♠Q"), output_style: Black.bold(), value: 10},
        ], 21
    },
    twoqk = {
        vec![
            Card{representation: String::from("♣K"), output_style: Black.bold(), value: 10},
            Card{representation: String::from("♥2"), output_style: Red.bold(), value: 2},
            Card{representation: String::from("♠Q"), output_style: Black.bold(), value: 10},
        ], 22
    },
    aaaa = {
        vec![
            Card{representation: String::from("♣A"), output_style: Black.bold(), value: 11},
            Card{representation: String::from("♥A"), output_style: Red.bold(), value: 11},
            Card{representation: String::from("♠A"), output_style: Black.bold(), value: 11},
            Card{representation: String::from("♦A"), output_style: Red.bold(), value: 11},
        ], 14
    },
    qkaa = {
        vec![
            Card{representation: String::from("♣K"), output_style: Black.bold(), value: 10},
            Card{representation: String::from("♠Q"), output_style: Black.bold(), value: 10},
            Card{representation: String::from("♣A"), output_style: Black.bold(), value: 11},
            Card{representation: String::from("♥A"), output_style: Red.bold(), value: 11},
        ], 22
    },
    jqk = {
        vec![
            Card{representation: String::from("♥J"), output_style: Red.bold(), value: 10},
            Card{representation: String::from("♣K"), output_style: Black.bold(), value: 10},
            Card{representation: String::from("♠Q"), output_style: Black.bold(), value: 10},
        ], 30
    },
    numbers = {
        vec![
            Card{representation: String::from("♣2"), output_style: Black.bold(), value: 2},
            Card{representation: String::from("♠3"), output_style: Black.bold(), value: 3},
            Card{representation: String::from("♣4"), output_style: Black.bold(), value: 4},
            Card{representation: String::from("♠5"), output_style: Black.bold(), value: 5},
            Card{representation: String::from("♣6"), output_style: Black.bold(), value: 6},
            Card{representation: String::from("♣7"), output_style: Black.bold(), value: 7},
            Card{representation: String::from("♠8"), output_style: Black.bold(), value: 8},
            Card{representation: String::from("♣9"), output_style: Black.bold(), value: 9},
            Card{representation: String::from("♠10"), output_style: Black.bold(), value: 10},
        ], 54
    })]
    fn test_calculate_score_returns_correct_result(hand: Vec<Card>, result: u32) {
        assert_eq!(calculate_score(&hand), result);
    }

    #[parameterized(
        base_case_zero_aces = {30, 0, 30},
        base_case_score = {20, 0, 20},
        induction_step_aces = {30, 1, 20},
        induction_step_score = {20, 1, 20}
    )]
    fn test_replace_ace_values_returns_correct_score(score: u32, remaining_aces: u32, result: u32) {
        assert_eq!(replace_ace_values(score, remaining_aces), result);
    }

    #[parameterized(
        zero_aces = {
            vec![
                Card{representation: String::from("♣2"), output_style: Black.bold(), value: 2},
            ], 0
        },
        one_ace = {
            vec![
                Card{representation: String::from("♣A"), output_style: Black.bold(), value: 11},
            ], 1
        },
        two_aces = {
            vec![
                Card{representation: String::from("♣A"), output_style: Black.bold(), value: 11},
                Card{representation: String::from("♥A"), output_style: Black.bold(), value: 11},
            ], 2
        },
    )]
    fn test_nr_of_aces_in_hand_returns_expected_output(hand: Vec<Card>, result: u32) {
        assert_eq!(nr_of_aces_in_hand(&hand), result);
    }

    #[parameterized(
        user_lost = {22, 50, FinalScore::DealerWins},
        dealer_wins = {20, 21, FinalScore::DealerWins},
        draw = {20, 20, FinalScore::Draw},
        draw_blackjack = {21, 21, FinalScore::Draw},
        user_blackjack = {21, 20, FinalScore::UserWins},
        user_wins = {15, 22, FinalScore::UserWins},
    )]
    fn test_compare_scores_returns_expected_output(user_score: u32, dealer_score: u32, result: FinalScore) {
        assert_eq!(compare_scores(user_score, dealer_score), result);
    }

    #[parameterized(
        user_won = {
            vec![
                Card{representation: String::from("♣A"), output_style: Black.bold(), value: 11},
                Card{representation: String::from("♣J"), output_style: Black.bold(), value: 10},
            ],
            vec![
                Card{representation: String::from("♣10"), output_style: Black.bold(), value: 10},
                Card{representation: String::from("♣J"), output_style: Black.bold(), value: 10},
            ],
            FinalScore::UserWins
        },
        dealer_won = {
            vec![
                Card{representation: String::from("♣10"), output_style: Black.bold(), value: 10},
                Card{representation: String::from("♣J"), output_style: Black.bold(), value: 10},
            ],
            vec![
                Card{representation: String::from("♣A"), output_style: Black.bold(), value: 11},
                Card{representation: String::from("♣J"), output_style: Black.bold(), value: 10},
            ],
            FinalScore::DealerWins
        },
        draw = {
            vec![
                Card{representation: String::from("♣10"), output_style: Black.bold(), value: 10},
                Card{representation: String::from("♣J"), output_style: Black.bold(), value: 10},
            ],
            vec![
                Card{representation: String::from("♣K"), output_style: Black.bold(), value: 10},
                Card{representation: String::from("♣Q"), output_style: Black.bold(), value: 10},
            ],
            FinalScore::Draw
        },
    )]
    fn test_check_final_score_returns_expected_output(player_hand: Vec<Card>, dealer_hand: Vec<Card>, result: FinalScore) {
        assert_eq!(check_final_score(&player_hand, &dealer_hand), result);
    }
}