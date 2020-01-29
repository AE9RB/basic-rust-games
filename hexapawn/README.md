# Hexapawn

https://en.wikipedia.org/wiki/Hexapawn

A game invented by Martin Gardner in the 1960s.
He proposed using heuristic artificial intelligence
implemented on a mechanical computer.

It's your three pawns against the computer's pawns on a 3x3 board.
You are White.
You lose when you can no longer make a move.
You lose when the computer reaches the third rank.

```
+---+---+---+   +---+---+---+
| X | X | X |   | 7 | 8 | 9 |
+---+---+---+   +---+---+---+
|   |   |   |   | 4 | 5 | 6 |
+---+---+---+   +---+---+---+
| O | O | O |   | 1 | 2 | 3 |
+---+---+---+   +---+---+---+
```

The rules for an AI like this are very straightforward.
When a new board pattern is encountered it is stored in a HashMap with a vector of all possible moves.
The computer selects its moves randomly from the vector of moves.
When the computer loses a game, the last move selected is removed so it can not be selected again.
If no moves remain for a pattern then the move on the previous board pattern is removed.
