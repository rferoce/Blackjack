use rand::Rng;
use crate::models::Card;

pub fn draw_card(hand: &mut Vec<Card>, deck: &mut Vec<Card>) {
    let card = get_card_from_deck(deck);
    hand.push(card);
}

pub fn get_card_from_deck(deck: &mut Vec<Card>) -> Card {
    let card_index = rand::rng().random_range(0..deck.len());
    deck.remove(card_index)
}

#[cfg(test)]
mod tests {
    use ansi_term::Color::{Black, Red};
    use super::*;

    #[test]
    fn test_draw_card_adds_card_from_deck_to_hand() {
        let mut deck = vec![
            Card{representation: String::from("♣2"), output_style: Black.bold(), value: 2},
            Card{representation: String::from("♣3"), output_style: Black.bold(), value: 3},
            Card{representation: String::from("♥4"), output_style: Red.bold(), value: 4},
        ];
        let mut hand = vec![
            Card{representation: String::from("♣5"), output_style: Black.bold(), value: 5},
        ];

        draw_card(&mut hand, &mut deck);

        assert_eq!(deck.len(), 2);
        assert_eq!(hand.len(), 2);
    }

    #[test]
    fn test_get_card_from_deck_returns_original_card() {
        let mut deck = vec![
            Card{representation: String::from("♣2"), output_style: Black.bold(), value: 2},
            Card{representation: String::from("♣3"), output_style: Black.bold(), value: 3},
            Card{representation: String::from("♣4"), output_style: Black.bold(), value: 4},
        ];

        let next_card = get_card_from_deck(&mut deck);
        assert_eq!(deck.len(), 2);
        let result :Vec<&Card> = deck.iter().filter(|&c| c.value == next_card.value).collect();
        assert_eq!(result.len(), 0);
    }
}