fn random_int() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    seed ^= seed << 21;
    seed ^= seed >> 35;
    seed ^= seed << 4;
    seed
}

fn main() {
    println!("Hello, user!");

    println!("Try to guess a number from 1 to 100");
    let number = (random_int() % 100 + 1) as u8;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: u8 = input.trim().parse().unwrap();

        if input == number {
            println!("You guessed the number! congratulations!");
            break;
        }
        if input < number {
            println!("My number is greater than yours...")
        } else {
            println!("My number is less than yours...")
        }
    }
}
