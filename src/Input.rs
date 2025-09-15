use std::io;
use std::thread;
use std::time::Duration;

pub fn input() {
    println!("Please insert your first number to operate: ");

    let mut add1 = String::new();
    io::stdin()
        .read_line(&mut add1)
        .expect("Failed to read line");

    let add1: i32 = add1.trim().parse().expect("Error, please insert a valid number.");

    println!("Please insert your second number to operate: ");

    let mut add2 = String::new();
    io::stdin()
        .read_line(&mut add2)
        .expect("Failed to read line");

    let add2: i32 = add2.trim().parse().expect("Error, please insert a valid number.");

    println!("You have selected: {} and {}", add1, add2);
    thread::sleep(Duration::from_secs(1));

    println!("Please insert your operation symbol(+,-,/,*): ");

    let mut proc = String::new();
    io::stdin()
        .read_line(&mut proc)
        .expect("Failed to read line");

    let proc = proc.trim();  


    println!("Click enter to confirm...");
    let mut dummy = String::new();
    std::io::stdin().read_line(&mut dummy).unwrap();

    thread::sleep(Duration::from_secs(1));
    println!("Processing.");
    thread::sleep(Duration::from_millis(1000));
    println!("Processing..");
    thread::sleep(Duration::from_millis(1000));
    println!("Processing...");
    thread::sleep(Duration::from_millis(1500));
    println!("Calculating.");
    thread::sleep(Duration::from_millis(700));
    println!("Calculating..");
    thread::sleep(Duration::from_millis(700));
    println!("Calculating...");
    thread::sleep(Duration::from_millis(1500));
    println!("Finalizing...");
    thread::sleep(Duration::from_millis(700));

    let result: i32 = match proc {
        "+" => add1 + add2,
        "-" => add1 - add2,
        "*" => add1 * add2,
        "/" => add1 / add2,
        _ => 0,
    };
    println!("The result is: {}", result);

    thread::sleep(Duration::from_secs(3));
}