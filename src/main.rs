use rand::Rng;
mod input;
use std::io;

fn main() {
    loop {
    println!("Please insert your code, press 3 for help and 0 to quit: ");

    let mut game = String::new();
    io::stdin()
        .read_line(&mut game)
        .expect("Failed to read line");

    let game =game.trim().parse().expect("Error, please insert a valid number.");

    match game {
        1 =>{
            let mut rng = rand::rng();
            let random = rng.random_range(1..=10000);
            println!("Random number: {}", random); 
        }
        2 => input::input(),
        3 => {
                println!(
                    "Help Menu: 
1: Random number generator (1-10000) 
2: 2 number calculator 
3: Help Menu 
0: Exit"
                );
                println!("Press Enter to return to menu...");
                let mut dummy = String::new();
                io::stdin().read_line(&mut dummy).unwrap();
                continue; // go back to menu
            }
        0 => {
        println!("Thank you come again!");
        break;
        }
        _ => println!("Error, please select a valid number."),
        
    }

}} 
