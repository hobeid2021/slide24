# Sliding Puzzle
## Description
A basic sliding puzzle game in Rust using [macroquad](https://macroquad.rs/). 
When the game starts it loads the images in the img/ directory in alphanumerical order.
While the current version of the game is made for images which are 600x600, if an image of other size were to be loaded, the program will resize the image using bilinear filtering to 600x600. Note that this resizing might not look as good as if it were resized using a more advanced external program.

## How to run
Install rust and then run `cargo build` to install dependancies and build the application and then `cargo run` to run the executable.
The game starts in an already solved state with the empty tile at the bottom right corner, press 'S' to shuffle the board. To move a tile, click on it (a green border will appear) then click on an empty space to move it. The puzzle is solved when the configuration of tiles matches the base state. To ensure the board is solvable, amount of inversions of the shuffled board is counted, if the amount of inversions are even it is considered solvable, otherwise odd. Pressing space will show the puzzle grid as numbered tiles, with 1-8 being elements of the puzzle and 0 representing the empty tile.
