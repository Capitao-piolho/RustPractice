use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let random = rng.random_range(1..=10000);
    println!("Random number: {}", random);
}
