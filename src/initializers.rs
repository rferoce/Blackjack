use std::io;
use ansi_term::Color::{Black, Red};
use ansi_term::Style;
use rand::Rng;
use crate::models::Card;

pub fn init_player() -> String {
    let mut user_input = String::new();

    println!("What is your name?");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read your name");

    let name: String = String::from(user_input.trim());
    println!("\nHello {}!", name);
    name

}

pub fn init_budget() -> u32 {
    let total_money = rand::rng().random_range(500..1000);
    println!("your total budget for today's game equals {} EUR", total_money);
    total_money
}

fn init_card(representation: String, output_style: Style, value: u32) -> Card {
    Card { representation, output_style, value }
}

pub fn init_deck() -> Vec<Card> {
    let numbers = (2..11).map(|x| x.to_string()).collect::<Vec<String>>();
    let non_numbers = ['J', 'Q', 'K', 'A'];
    let symbols = [
        (String::from("♣"), Black.bold()),
        (String::from("♥"), Red.bold()),
        (String::from("♠"), Black.bold()),
        (String::from("♦"), Red.bold())
    ];
    let mut deck :Vec<Card>  = Vec::new();

    for (x, y) in symbols.iter() {
        for z in numbers.iter() {
            deck.push(init_card(format!("{x}{z}"), *y, z.parse::<u32>().unwrap()));
        }
        for z in non_numbers.iter() {
            let card_value = match z {
                'A' => 11,
                _ => 10
            };
            deck.push(init_card(format!("{x}{z}"), *y, card_value));
        }
    }
    deck
}

pub fn init_game(player_name: &String, total_money: u32) -> (u32, Vec<Card>) {
    let deck = init_deck();
    let mut user_input = String::new();
    loop {
        println!("\nWhat is the amount of money you would like to play with?");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the amount of money you would like to play with");

        let bet :u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            },
        };

        match bet {
            x if x < 1 => {
                println!("{x}");
                println!("You entered an invalid amount of money!");
                println!("Please enter a valid amount of money to play with!")
            },
            x if x < total_money => {
                println!("You will be playing with {} EUR, good luck {}!\n", bet, *player_name);
                return (bet, deck);
            },
            x if x == total_money => {
                println!("You will play ALL-IN ({} EUR), good luck {}!\n", bet, *player_name);
                return (bet, deck);
            },
            x if x > total_money => {
                println!("It is not possible to play with {} EUR, because your budget equals {}!", bet, total_money);
                println!("Please enter a valid amount of money to play with!")
            },
            _ => println!("Unexpected behaviour, we will try again!")
        }
    }
}
