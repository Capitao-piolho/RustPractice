use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let number1 = rng.gen_range(1..100);
    let number2 = rng.gen_range(1..100);
    println!("The solution of {} + {} equals {}",
    number1,
    number2,
    number1 + number2);
}
