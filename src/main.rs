use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0,21);

    let mut count = 0; //create counter for loop

    loop { //create loop for guesses

        println!("Please guess a number between 1 and 20.");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { //take string guess, remove whitespace, convert to int
            Ok(num) => num, 
            Err(_) => continue, //_ catchall ignores non-number, continue goes to top of loop to ask for number to be entered
        };

        match guess.cmp(&secret_number) { //match guess vs secret number, return hints to user
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { //if user is correct, print correct, count
                println!("Correct! \n It took {} guessses!",count);
                break; //exit loop, program ends
            }
        }
        count = count + 1 //update counter after each guess
    }
}
