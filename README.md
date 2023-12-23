# CNCT4Game

Hello! This is a small project I am using to learn rust and experiment with building command line tools. This is a simple game of connect 4 that can be played in the terminal. 

As of now, the game can be played locally against another player.

The board size is 6 rows and 7 columns, which when `target/release/cnct4game` is run will be printed to the terminal as an array of zeros

```
0000000
0000000
0000000
0000000
0000000
0000000

Player 1, choose a column to play in!
```

Then, a number 1-7 can be chosen, corresponding to the column to play in, and the change will be reflected by an output in the terminal.

When a sequence of 4 is detected, a winning message will be displayed and the script will end. Or, if there are no zeros detected in the board, a stalemate message will be displayed and the script will end.

The size of the board can be changed using the `-c` and `-r` arguments, for example 
```bash
cnct4game -r 8 -c 10
```



## Future Features

* Add optional arguments that can change the size of the board (done!)
* Fix bugs when making board less than 4 rows or columns
* Add an AI to play against optionally