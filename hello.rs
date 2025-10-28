fn main() {
    println!("rust.");
    let name = "rustacean";
    println!(" hi hru {}?", name);

    let x = 5;
    let y = 10;
    println!("Quick math: {} + {} = {}", x, y, x + y);

    if x < y {
        println!("looks like {} is smaller than {}.", x, y);
    } else {
        println!("hmm, {} is not smaller than {}.", x, y);
    }

    println!("bye");
}