# Life

## Basic Rules

This is Conway's Game of Life.<br/>
https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life

The rules are simple.
There is a grid where each cell is empty or alive.
If a live cell has zero or one neighbors it dies of loneliness.
If a live cell has four or more neighbors it dies from overpopulation.
If a live cell has two or three neighbors it survives.
If an empty cell has exactly three neighbors one is born.

The grid is iterated in its entirety.
Births and deaths happen simultaneously.
In other words, the application of rules is a pure function.
