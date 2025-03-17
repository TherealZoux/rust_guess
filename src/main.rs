use rand::Rng;
use std::io; // Import the random number generator

fn main() {
    let mut rng = rand::rngs::ThreadRng::default(); // Updated method
    let guess_num: i32 = rng.random_range(0..=10); // Updated method
    let mut f_player = String::new();
    let mut s_player = String::new();

    // Input first player name
    println!("Input the first player's name: ");
    io::stdin()
        .read_line(&mut f_player)
        .expect("Failed to read name!");
    let f_player = f_player.trim();

    // Input second player name
    println!("Input the second player's name: ");
    io::stdin()
        .read_line(&mut s_player)
        .expect("Failed to read name!");
    let s_player = s_player.trim();

    println!("The target number has been set! Try to guess it (0-10).");

    loop {
        let mut f_player_g = String::new();
        let mut s_player_g = String::new();

        // First player guess input
        println!("{}, please input your guess (0-10):", f_player);
        io::stdin()
            .read_line(&mut f_player_g)
            .expect("Failed to get input");
        let f_player_g: Result<i32, _> = f_player_g.trim().parse();

        // Second player guess input
        println!("{}, please input your guess (0-10):", s_player);
        io::stdin()
            .read_line(&mut s_player_g)
            .expect("Failed to get input");
        let s_player_g: Result<i32, _> = s_player_g.trim().parse();

        // Check if input parsing was successful
        match (f_player_g, s_player_g) {
            (Ok(f_guess), Ok(s_guess)) => {
                if f_guess == guess_num && s_guess == guess_num {
                    println!(
                        "Both {} and {} guessed correctly! It's a tie!",
                        f_player, s_player
                    );
                } else if f_guess == guess_num {
                    println!("{} wins! Good job!", f_player);
                    break;
                } else if s_guess == guess_num {
                    println!("{} wins! Good job!", s_player);
                    break;
                } else {
                    println!("You both lost! Try again.");
                }
            }
            _ => println!("Invalid input detected! Please enter numbers only."),
        }
    }
}

/*
 * so, we have 2 players
 * and constant number
 * should every player guess the nuber and the correct guess will will
 *
 *
 * */
