enum PinState {
    High(u8),
    Low(u8),
}

fn main() {
    let state = PinState::High(13);
    match state {
        PinState::High(pin) => println!("pin {} is HIGH", pin),
        PinState::Low(pin)  => println!("pin {} is LOW", pin),
    }
}
