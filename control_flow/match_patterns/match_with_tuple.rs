fn main() {
    let pin = (1, true); // (pin_number, is_high)
    match pin {
        (1, true)  => println!("pin 1 is HIGH"),
        (1, false) => println!("pin 1 is LOW"),
        (2, true)  => println!("pin 2 is HIGH"),
        (2, false) => println!("pin 2 is LOW"),
        _          => println!("unknown pin"),
    }
}
