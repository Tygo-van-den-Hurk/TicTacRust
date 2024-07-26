use std::io::{self, Write};

/// Stores the version of the program globally in case it's needed.
const VERSION: &str = "v0.0.1";

fn main() {
    print!(
"Hello to TicTacRust!
    
You are running version: {VERSION}

To get Started we'll first ask the players for their name so that we can refer to them by name. Please state the name of player 1: "
    );

    let mut player1_name = String::new();

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut player1_name)
        .expect("Failed to read line");

    let player1_name = player1_name.trim();
    let mut player2_name = String::new();

    print!("Hello, {player1_name}! Who will you be fighting? Enter their name here: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut player2_name)
        .expect("Failed to read line");

    let player2_name = player2_name.trim();

    print!("
Okay then, {player2_name}, you've been challenged!
To enter where you'd like to set your mark you can give a coordinate. You can do that one of these ways:
  - (x,y)
  - x,y
  - x y
  
where x is a character, and y is a number. In case you need a refresher of highschool math, the x coordinate is the horizontal placement, while y is the vertical placement. Now that you know that you're ready to rumble!
"
    );
}
