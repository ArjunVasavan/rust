fn match_range(n: i32) -> &'static str {
    match n {
        1..=5  => "small",
        6..=10 => "medium",
        _      => "large",
    }
}

fn main() {
    println!("{}", match_range(4));
    println!("{}", match_range(8));
    println!("{}", match_range(99));
}
