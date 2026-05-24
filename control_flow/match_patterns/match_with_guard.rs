fn main() {
    let x = 7;
    match x {
        n if n < 0  => println!("negative"),
        n if n == 0 => println!("zero"),
        n if n > 0  => println!("positive: {}", n),
        _           => println!("unknown"),
    }
}
