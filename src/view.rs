use std::io;
use crate::models::Card;

pub fn show_introduction() {
    println!("Welcome to Blackjack!");
    println!("The rules are simple. Don't lose to the dealer!");
    println!("Before we start, I would like to know who you are.");
}

pub fn ask_for_next_step() -> String {
    let mut user_input = String::new();
    println!("Decide your next step: Stand or Hit (S/H)?");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read your next step");
    user_input.trim().to_lowercase()
}

pub fn show_end_user_input_cycle() {
    println!("-------------------------");
}

pub fn show_invalid_input_error(inp: &str) {
    println!("Invalid input: {}!", inp);
}

pub fn show_stats(player_hand: &Vec<Card>, dealer_hand: &Vec<Card>) {
    show_hand(player_hand, "Your");
    show_hand(dealer_hand, "Dealer");
}

pub fn show_hand(hand: &Vec<Card>, owner: &str) {
    println!("{} cards:", owner);
    for card in hand.iter() {
        print!("{} ", card.output_style.paint(&card.representation));
    }
    println!("\n");
}

pub fn show_user_won_round(remaining_money: u32) {
    println!("\nYou have won, so you doubled your bet!");
    println!("Your current amount of money equals {} EUR", remaining_money);
}

pub fn show_user_played_draw_this_round(remaining_money: u32) {
    println!("\nIt's a draw!");
    println!("Your amount of money will stay unchanged: {} EUR", remaining_money);
}

pub fn show_user_lost_round(remaining_money: u32, add_new_line: bool) {
    if add_new_line {
        println!("\n")
    }
    println!("You lost, try again!");
    println!("Your current amount of money equals {} EUR", remaining_money);
}

pub fn ask_to_play_again() -> String{
    println!("Do you want to play again? (Y/n)?");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read your input");
    user_input.trim().to_lowercase()
}

pub fn show_lost_all_money() {
    println!("\nOh no, you lost all your money!");
    println!("Better luck next time!");
}

pub fn show_end_game(player: &String, total_money: &u32) {
    println!("\n{}, you ended the game with a total amount of {} EUR", *player, *total_money);
    println!("Thank you for playing Blackjack!");
    println!("See you next time!")
}
