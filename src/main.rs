use rand::Rng;
mod input;
use std::io;
use std::thread;
use std::time::Duration;


fn main() {
    loop { //loop so the user can use multiple functions without restarting the terminal
    println!("Please insert the desired game, press 3 for help and 0 to quit: ");

    let mut game = String::new(); //user input
    io::stdin()
        .read_line(&mut game)
        .expect("Failed to read line");

    let game =game.trim().parse().expect("Error, please insert a valid number.");

    match game { //simple text GUI
        1 =>{ //random number generator
            println!("Generating random number...");
            thread::sleep(Duration::from_millis(700));
            let mut rng = rand::rng();
            let random = rng.random_range(1..=10000);
            println!("Random number: {}", random); 
        }
        2 => input::input(), //calls input.rs
        3 => { //HELP MENU
                println!(
                    "Help Menu: \n1: Random number generator (1-10000) \n2: 2 number calculator \n3: Help Menu \n0: Exit"
                );
                thread::sleep(Duration::from_secs(1));
                continue; // go back to menu
            }
        0 => { //ending the terminal
        println!("Thank you come again!");
        break;
        }
        _ => println!("Error, please select a valid number."),
        
    }

}} 
