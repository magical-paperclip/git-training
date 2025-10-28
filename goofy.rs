// A goofy Rust program that does silly things

fn main() {
    println!("Welcome to the Goofy Rust Program!");

    let mut counter = 0;

    // A loop that counts to 5 but does something silly
    while counter < 5 {
        println!("Counter is at: {}", counter);
        if counter % 2 == 0 {
            println!("Goofy says: Even numbers are cool!");
        } else {
            println!("Goofy says: Odd numbers are wacky!");
        }
        counter += 1;
    }

    // A function that returns a random "goofy" string
    let goofy_message = get_goofy_message();
    println!("Goofy message of the day: {}", goofy_message);
}

fn get_goofy_message() -> &'static str {
    let messages = [
        "Rustaceans are the best!",
        "Borrow checker is my best friend!",
        "I love lifetimes!",
        "Enums are enum-azing!",
        "Cargo is my co-pilot!",
    ];

    // Pick a random message (for simplicity, use modulo)
    let index = 42 % messages.len(); // 42 is the answer to everything, right?
    messages[index]
}