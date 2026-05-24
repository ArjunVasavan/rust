fn is_valid_pin(pin: u8) -> bool {
    pin <= 13
}

fn main() {
    let pin = 5;
    if is_valid_pin(pin) {
        println!("pin {} is valid", pin);
    } else {
        println!("pin {} is invalid", pin);
    }
}
