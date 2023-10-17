use rand::prelude::*;
use std::io;

fn play(secret_number: i32, mut guesses_count: i32) {
    let mut player_guess = String::new();

    io::stdin()
        .read_line(&mut player_guess)
        .expect("Error reading player input");

    let guess = player_guess
        .trim()
        .parse::<i32>()
        .expect("Error parsing player input");

    guesses_count += 1;

    if guesses_count < 5 {
        if guess < 1 || guess > 100 {
            println!(
                "\nYou've taken {guesses_count} guess(es): {guess} is out of range, guess between 1 and 100 >"
            );

            play(secret_number, guesses_count);
        } else if guess < secret_number {
            println!(
                "\nYou've taken {guesses_count} guess(es): {guess} is too low, guess higher >"
            );

            play(secret_number, guesses_count);
        } else if guess > secret_number {
            println!(
                "\nYou've taken {guesses_count} guess(es): {guess} is too high, guess lower >"
            );

            play(secret_number, guesses_count);
        } else {
            println!("\nCongrats, you got it! 😎 The secret number was {secret_number} 🙃");
        }
    }

    if guesses_count == 5 {
        if guess == secret_number {
            println!("\nCongrats, you got it on the fifth try! 😅 The secret number was {secret_number} 🙃");
        } else {
            println!("\nOops, you lose 😥 This was your fifth and final try. The secret number was {secret_number} 🙃");
            println!("Do you want to play again? 🙂 Press y or n... >");

            let mut player_choice = String::new();

            io::stdin()
                .read_line(&mut player_choice)
                .expect("Error reading player input");

            let choice = player_choice.trim();

            if choice == "y" {
                println!("\nPlease, guess the secret number in a range between 1 and 100 >");

                let secret_number = thread_rng().gen_range(1..101);
                guesses_count = 0;

                play(secret_number, guesses_count);
            } else {
                println!("\nGoodbye and good luck! 🙂");
            }
        }
    }
}

fn main() {
    println!("\nWelcome to the guessing game! 🙂");
    println!("\nYou can try your luck up to 5 times. Let's get started!");
    println!("\nPlease, guess the secret number in a range between 1 and 100 >");

    let secret_number = thread_rng().gen_range(1..101);
    let guesses_count = 0;

    play(secret_number, guesses_count);
}
