use crate::helpers::score::calculate_score;
use crate::models::Card;
pub fn lost(hand: &Vec<Card>) -> bool {
    calculate_score(hand) > 21
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
            Card{representation: String::from("♥A"), output_style: Red.bold(), value: 11},
        ], false
    },
    kaa = {
        vec![
            Card{representation: String::from("♣K"), output_style: Black.bold(), value: 10},
            Card{representation: String::from("♥A"), output_style: Red.bold(), value: 11},
            Card{representation: String::from("♠A"), output_style: Black.bold(), value: 11},
        ], false
    },
    qka = {
        vec![
            Card{representation: String::from("♣K"), output_style: Black.bold(), value: 10},
            Card{representation: String::from("♥A"), output_style: Red.bold(), value: 11},
            Card{representation: String::from("♠Q"), output_style: Black.bold(), value: 10},
        ], false
    },
    twoqk = {
        vec![
            Card{representation: String::from("♣K"), output_style: Black.bold(), value: 10},
            Card{representation: String::from("♥2"), output_style: Red.bold(), value: 2},
            Card{representation: String::from("♠Q"), output_style: Black.bold(), value: 10},
        ], true
    },
    aaaa = {
        vec![
            Card{representation: String::from("♣A"), output_style: Black.bold(), value: 11},
            Card{representation: String::from("♥A"), output_style: Red.bold(), value: 11},
            Card{representation: String::from("♠A"), output_style: Black.bold(), value: 11},
            Card{representation: String::from("♦A"), output_style: Red.bold(), value: 11},
        ], false
    },
    qkaa = {
        vec![
            Card{representation: String::from("♣K"), output_style: Black.bold(), value: 10},
            Card{representation: String::from("♠Q"), output_style: Black.bold(), value: 10},
            Card{representation: String::from("♣A"), output_style: Black.bold(), value: 11},
            Card{representation: String::from("♥A"), output_style: Red.bold(), value: 11},
        ], true
    })]
    fn test_lost_returns_correct_result(hand: Vec<Card>, result: bool) {
        assert_eq!(lost(&hand), result);
    }
}
