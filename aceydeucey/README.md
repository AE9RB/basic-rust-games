# Acey Deucey

https://en.wikipedia.org/wiki/Acey_Deucey_(card_game)

In this game the dealer will draw three playing cards.
Two of these cards are revealed to the player.
The player bets on whether they think the rank of the third card is between those two.

In the original BASIC version, three card ranks were simply drawn directly from a random source.
Since there are four suits and we only need three cards it's a good illusion.
This version shuffles a deck of 52 cards so the odds match real life.

There are many variations to betting, deck management, and the number of players.
We're going to do the bare minimum. This will provide a starting point for
you to develop into a more sophisticated version with your preferred rules.

```
 ***************************
 *  A C E Y   D E U C E Y  *
 ***************************

Two cards are dealt face up and a third face down.
You win if the rank of the third card is between the first two.

You have 100 coins.
The deal: [Queen] [Jack] [???]
Your Bet? 0
Chicken.

You have 100 coins.
The deal: [Queen] [6] [???]
Your Bet? 75
You win!: [Queen] [6] [9]

You have 175 coins.
The deal: [6] [2] [???]
Your Bet? 175
You lose: [6] [2] [7]
You are out of coins.
Goodbye.
```
