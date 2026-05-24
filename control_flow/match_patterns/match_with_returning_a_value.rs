fn main() {
    let x = 5;
    let result = match x {
        1..=5  => "low",
        6..=10 => "high",
        _      => "unknown",
    };
    println!("{}", result);
}
