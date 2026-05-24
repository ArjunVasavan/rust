fn pin_mode(mode: u8) -> &'static str {
    match mode {
        0 => "INPUT",
        1 => "OUTPUT",
        2 => "INPUT_PULLUP",
        _ => "UNKNOWN",
    }
}

fn main() {
    println!("{}", pin_mode(0));
    println!("{}", pin_mode(1));
    println!("{}", pin_mode(2));
}
