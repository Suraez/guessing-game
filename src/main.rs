use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let secret_num: i32 = rand::thread_rng().gen_range(1..=100);


    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: i32 = user_input.trim().parse().expect("Please type a number!");

    match user_input.cmp(&secret_num) {
        Ordering::Less => println!("The user number is lesser.."),
        Ordering::Greater => println!("The uesr number is greater.."),
        Ordering::Equal => println!("Yay, you matched it correctly")
    };

    println!("The secret number is {}",  secret_num);
    println!("You guessed: {user_input}");
}