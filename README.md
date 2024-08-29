>  A basic Tic Tac Toe CLI client.

# TicTacRust

A simple Tic Tac Toe game implemented in Rust for the command line interface (CLI). The game allows two players to take turns and place their symbols (X or O) on a 3x3 board. The first player to place three of their symbols in a row, column, or diagonal wins the game. If the board is filled without any player winning, the game ends in a draw.

## Features

- **Two-player mode:** Play against another person locally.
- **CLI interface:** The game runs entirely in the command line.
- **Simple and intuitive gameplay:** Players take turns inputting their move coordinates.

## How to Play

1. **Run the game:** Execute the compiled binary from your terminal.
2. **Input your move:** When prompted, input the row and column numbers (both between 0 and 2) where you want to place your symbol.
3. **Win or Draw:** The game will automatically detect if a player has won or if the game ends in a draw.

## Game Rules

- The game is played on a 3x3 grid.
- Players take turns to place their symbol (`X` or `O`) on the board.
- The first player to align three symbols in a row, column, or diagonal wins.
- If all cells are filled and no player has won, the game ends in a draw.

## Example Gameplay

```shell
Player 1 (X), enter your move (row and column): 0 0

X |   |  
---------
  |   |  
---------
  |   |  
```

```shell
Player 2 (O), enter your move (row and column): 1 1

X |   |  
---------
  | O |  
---------
  |   |  
```