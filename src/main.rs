mod initializers;
mod enums;
mod predicates;
pub mod helpers;
use crate::helpers::score::{calculate_score, check_final_score};
use crate::helpers::actions::draw_card;


mod view;
mod models;

use enums::FinalScore::{DealerWins, UserWins, Draw};
use crate::models::Card;
use crate::predicates::lost;
use crate::view::{ask_for_next_step, ask_to_play_again, show_end_game, show_end_user_input_cycle, show_hand, show_invalid_input_error, show_lost_all_money, show_stats, show_user_lost_round, show_user_played_draw_this_round, show_user_won_round};

fn main() {
    view::show_introduction();
    let player = initializers::init_player();
    let mut budget = initializers::init_budget();
    budget = start(&player, budget);
    show_end_game(&player, &budget);
}

fn start(player: &String, total_money: u32) -> u32 {
    let (bet, deck) = initializers::init_game(player, total_money);
    let final_money = play_round(&bet, deck, total_money);
    if final_money == 0 {
        show_lost_all_money();
    } else if decide_to_play_again() {
        return start(player, final_money);
    }
    final_money
}

fn play_round(bet: &u32, deck: Vec<Card>, total_money: u32) -> u32 {
    let (mut user_hand, mut dealer_hand, mut deck) = deal_cards(deck);
    play_user(&mut user_hand, &mut deck, &dealer_hand);

    if lost(&user_hand) {
        let remaining_money = total_money - *bet;
        show_user_lost_round(remaining_money, false);
        return remaining_money;
    }

    play_dealer(&mut dealer_hand, &mut deck, &user_hand);
    match check_final_score(&user_hand, &dealer_hand) {
        DealerWins => {
            let remaining_money = total_money - *bet;
            show_user_lost_round(remaining_money, true);
            remaining_money
        },
        Draw => {
            show_user_played_draw_this_round(total_money);
            total_money
        },
        UserWins => {
            let remaining_money = total_money + (2 * *bet);
            show_user_won_round(remaining_money);
            remaining_money
        }
    }

}

fn deal_cards(mut deck: Vec<Card>)-> (Vec<Card>, Vec<Card>, Vec<Card>) {
    let mut user_hand = vec![];
    let mut dealer_hand = vec![];

    draw_card(&mut user_hand, &mut deck);
    draw_card(&mut user_hand, &mut deck);
    draw_card(&mut dealer_hand, &mut deck);

    show_end_user_input_cycle();
    show_hand(&user_hand, "Your");
    show_hand(&dealer_hand, "Dealer");
    (user_hand, dealer_hand, deck)
}

fn play_user(
    player_hand: &mut Vec<Card>,
    deck: &mut Vec<Card>,
    dealer_hand: &Vec<Card>,
) {
    if lost(player_hand) || calculate_score(player_hand) == 21 {
        return;
    }

    let next_step = ask_for_next_step();

    match &next_step[..] {
        "s" => {
            show_end_user_input_cycle();
        },
        "h" => {
            show_end_user_input_cycle();
            draw_card(player_hand, deck);
            show_stats(player_hand, dealer_hand);
            play_user(player_hand, deck, dealer_hand);
        }
        x => {
            show_invalid_input_error(x);
            play_user(player_hand, deck, dealer_hand);
        }
    }
}


fn play_dealer(
    dealer_hand: &mut Vec<Card>,
    deck: &mut Vec<Card>,
    player_hand: &Vec<Card>,
) {
    let player_score = calculate_score(player_hand);
    while !lost(dealer_hand) && calculate_score(dealer_hand) < player_score {
        draw_card(dealer_hand, deck);
        show_stats(player_hand, dealer_hand);
    }
}

fn decide_to_play_again() -> bool {
    match ask_to_play_again() {
        x if x != "y" && x != "n" => {
            show_invalid_input_error(&x);
            decide_to_play_again()
        },
        y => y == "y",
    }
}
