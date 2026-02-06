# Blackjack
This is my first application developed in Rust.

## Explanation

### Placing bets
After you entered your (nick)name, you will start with a random amount of money (EURO). The amount of money you will have during the entire run is an amount between 500 en 1000.
At the start of each game you are able to place your bet. The amount of money you would like to play with during a round equals at most the entire amount of money you received at the start of the application.

### Playing a round
At the start of each round, both your hand and the hand of the dealer are visualized so you can calculate what the score of your hand is.
If the total score of your hand exceeds 21 (BlackJack), you will lose this game immediately after which you can initialize a new game with a new total amount of money (as a result of your lose, you lost your bets).
After this step, you have to decide whether you would like to hit (press H key) a new card or stand (press S key) your hand.
In case you would like to hit a new card, a new round will be played while your hand will be extended with one card from the randomized deck.
In case you would like to stand your hand, the game will be continued by the dealer round.

### Dealer round
The dealer keeps adding cards from the deck to his hand until:
- The total score of his hand exceeds 21 (BlackJack)
- The total score of his hand equalizes the total score of the user
- The total score of his hand exceeds the total score of the user

After the dealer stopped adding cards, the final scores of both your hand and the dealer hand will be compared in order to decide the final result.
During the comparison between both your score and the dealer's score, one of the following cases might apply. 
- The total score of the user equals 21 (BlackJack) and the total score of the dealer does not equal 21 (BlackJack)
- The total score of the user is less than 21 and more than the total score of the dealer
- The total score of the user is less than 21 and the total score of the dealer is more than 21
  
You will win the game when one of these cases apply. As a result, you will receive twice your bets. 

During the comparison between both your score and the dealer's score, one of the following cases might apply.
- The total score of your hand exceeds 21 (BlackJack)
- The total score of your hand is less than the total score of the dealer, where the total score of the dealer is less than 22.

You will lose the game when one of these cases apply. As a result, you will lose your bets.

During the comparison between both your score and the dealer's score, the following case might apply.
- The total score of your hand equals the total score of the dealer's hand

It's a draw when this case applies. As a result, you will be returned your bets.

### Card values
Each card has it's own value during the game. The color of the card, does not affect the value of the card.
Only aces might switch from card value.
