use rand::seq::IteratorRandom;
use std::{
    io::{Write},
    process::exit,
};
use text_io::read;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
mod five_letter_words;

fn main() {
	println!("Welcome to Cliardle! Guess what 5 letter word your computer is thinking of!
type e and return to exit");
	loop {
		guessing_sequence();
	}
}

fn is_line_in_list(checking_for: &String) -> bool {
	for line in crate::five_letter_words::five_letter_words::five_letter_words.lines() {
    	if &line == checking_for {
    		return true;
    	}
	}
	// if no line in the file matches our string - line not in file
	false
}

// this function simply does what its name - it takes a file path as input and returns a random line from that file. Assuming our wordlist is the path, we'll you get a word from the wordlist.
fn random_line_from_list() -> String {
    	let lines = crate::five_letter_words::five_letter_words::five_letter_words
    	.lines()
    	.map(|l| l.to_string());

    	lines
        	.choose(&mut rand::thread_rng())
        	.unwrap()
}

fn guessing_sequence() {
	let word_to_guess = random_line_from_list();
	for i in 1..6 {
		println!("Guess number {}", i);
		// this is a loop that just keeps waiting for user input and parsing it. If e for exit, checks for that. Otherwise it tries to validate input, is input 5 letters.
		loop { 			
			let input: String = read!("{}\n");
			if input == "e" || input == "E" {
				exit_input();
			} else if input.len() == 5 {
				if is_line_in_list(&input) {
				//if we got here we have a valid word that needs to be checked against our word_to_guess.
					if input == word_to_guess {
						winning_sequence();
						break
					} else {
						// if user didn't win we need to check every letter for if it contains the letter and for placement.
						walk_word(word_to_guess.clone(), input);
						break
					}
					} else {
						println!("Your guess, {}, was not in wordlist. Try again!", input);
					}
				} else {
				println!("Your guess needs to be 5 characters long. Try again.");
				}
			}
	}
	println!("The word your computer was thinking about was {}.", word_to_guess );
}

fn exit_input() {
	println!("Are you sure you wanna exit? Y/N");
	let exit_input: String = read!("{}\n");
	if exit_input == "Y" || exit_input == "y" {
		exit(0);
	} 
}

fn winning_sequence() {
	println!("Winner!

Do you wanna exit? Press E and enter - otherwise press any other button and enter to continue.");
	let input: String = read!("{}\n");
	if input == "e" || input == "E" {
		exit_input();
	} 
}

// this function checks placements and contains on each letter from the user.
fn walk_word(word_to_guess: String, word_from_user: String) {
	let mut stdout = StandardStream::stdout(ColorChoice::Always);
	for c in word_from_user.chars() {
		if word_to_guess.contains(c) {
			if word_to_guess.find(c) == word_from_user.find(c) {
				stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
				write!(&mut stdout, "{}", c);
				stdout.reset();
			} else {
				stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
				write!(&mut stdout, "{}", c);
				stdout.reset();
			}
		} else {
			print!("_");
		}
	}
	println!("");
}

