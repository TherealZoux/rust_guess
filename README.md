# Multiplayer Number Guessing Game

## Description
This is a simple two-player number guessing game written in Rust. The program generates a random number between 0 and 10, and both players take turns guessing the number. The player who correctly guesses the number first wins. If both players guess correctly in the same round, it's a tie.

## Features
- Supports two players.
- Generates a random number between 0 and 10.
- Players take turns guessing the number.
- Declares a winner or a tie based on the guesses.
- Handles invalid input gracefully.

## Prerequisites
Ensure you have Rust installed on your system. If not, install it using:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation
1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/guessing-game.git
   ```
2. Navigate to the project directory:
   ```sh
   cd guessing-game
   ```
3. Build the project:
   ```sh
   cargo build
   ```

## Usage
Run the program using:
```sh
cargo run
```
Follow the prompts to enter player names and guess numbers.

## Example Gameplay
```
Input the first player's name:
> Alice

Input the second player's name:
> Bob

The target number has been set! Try to guess it (0-10).

Alice, please input your guess (0-10):
> 5

Bob, please input your guess (0-10):
> 3

You both lost! Try again.

Alice, please input your guess (0-10):
> 7

Bob, please input your guess (0-10):
> 7

Both Alice and Bob guessed correctly! It's a tie!
```

## Error Handling
- If a player enters a non-numeric value, the program prompts them to enter a valid number.
- Only numbers between 0 and 10 are accepted.

## Contributions
Contributions are welcome! Feel free to submit a pull request with improvements or bug fixes.

## Contact
For support or inquiries, contact [your-email@example.com](mailto:moazmohamed.dev@gmail.com).


