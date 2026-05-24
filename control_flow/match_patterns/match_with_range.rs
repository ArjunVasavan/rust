fn main() {
    let x = 6;
    match x {
        1..=5  => println!("small"),
        6..=10 => println!("medium"),
        _      => println!("large"),
    }
}
