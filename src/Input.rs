use std::io;
use std::thread;
use std::time::Duration;
use std::io::Write;
use num_rational::Ratio; //for checking if the division is repeating or not

pub fn input() { //function for option 2 on main.rs

    println!("Entering Calculator...");

    thread::sleep(Duration::from_millis(1200));

    println!("Please insert your first number to operate: "); //task

    let mut add1 = String::new(); //user input for first number
    io::stdin()
        .read_line(&mut add1)
        .expect("Failed to read line");

    let add1: f64 = add1.trim().parse::<f64>().expect("Error, please insert a valid number.");

    println!("Please insert your second number to operate: "); //task

    let mut add2 = String::new(); //user input for second number
    io::stdin()
        .read_line(&mut add2)
        .expect("Failed to read line");

    let add2: f64 = add2.trim().parse::<f64>().expect("Error, please insert a valid number."); 


    println!("You have selected: {} and {}", add1, add2); //confirmation of the choosen numbers
    thread::sleep(Duration::from_secs(1));

    println!("Please insert your operation symbol(+,-,/,*): "); //task

    let mut proc = String::new(); //user input for the operation
    io::stdin()
        .read_line(&mut proc)
        .expect("Failed to read line");

    let mut is_repeating: bool = false; //default value for repeating decimal check

    let proc = proc.trim();  

    if proc == "/"{
            let frac = Ratio::new_raw(add1 as i128, add2 as i128).reduced();   // 1/3 → 1/3                                 // só o denominador

    let mut d = frac.denom().clone();
    while d % 2 == 0 { d /= 2; }
    while d % 5 == 0 { d /= 5; }

    is_repeating = d != 1;
    }


    let result: f64 = match proc {
        "+" => add1 + add2,
        "-" => add1 - add2,
        "*" => add1 * add2,
        "/" => add1 / add2,
        _ =>{
            println!("Error, invalid operation symbol, returning to menu...");
            thread::sleep(Duration::from_secs(2));
            return;
        },
    };

    println!("press enter to confirm, press - to cancel"); //confirmation
    let mut dummy = String::new();
    std::io::stdin().read_line(&mut dummy).unwrap();
    match dummy.trim() { //cancelling operation
    "" => {},
    "-" => {
        println!("Operation cancelled, returning to menu...");
        thread::sleep(Duration::from_secs(2));
        return;}
     _ => {  // wildcard 
        println!("Invalid input, returning to menu...");
        thread::sleep(Duration::from_secs(2));
        return;
    }
    }

   let frames = ["Processing...", "Calculating...", "Finishing..."];

   for msg in frames { //simple loading animation
       print!("\r{:<15}", msg);
       io::stdout().flush().unwrap();
       thread::sleep(Duration::from_millis(700)); 
   }

   fn last_digit(result: f64, dec_places: usize) -> char {
    // cria string com o número arredondado
    let mut s = format!("{:.*}", dec_places, result);      // ex.: "0.3333333333"
    // tira zeros e ponto supérfluos
    s = s.trim_end_matches('0').trim_end_matches('.').into();
    // devolve o último carácter
    s.chars().last().unwrap_or('0')                        // fallback in extreme cases
}

// exemplo de uso dentro do teu código
let digit = last_digit(result, 10);

let mut final_r = format!("{:.10}", result); // trimming to 10 decimal places
final_r = final_r.trim_end_matches('0').trim_end_matches('.').into(); // remove
if final_r.ends_with('.') { final_r.pop();}

   print!("\r{:<15}", ""); // Clear the line

    println!("\rThe result is: {:.10}\n", final_r); //result with 10 decimal places

   if is_repeating == true{
        println!("Note: The result is a periodic decimal, with it's period probably being {}", digit);
    }

    thread::sleep(Duration::from_secs(3));
    println!("Returning to menu...");
    thread::sleep(Duration::from_millis(100));
}
