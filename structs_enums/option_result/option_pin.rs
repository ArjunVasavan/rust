// returns Some(value) if pin valid, None if not
fn read_pin(pin: u8) -> Option<f32> {
    if pin <= 13 {
        Some(3.3) // valid pin returns voltage
    } else {
        None      // invalid pin returns nothing
    }
}

fn main() {
    // valid pin
    match read_pin(5) {
        Some(v) => println!("pin voltage = {}v", v),
        None    => println!("invalid pin"),
    }

    // invalid pin
    match read_pin(99) {
        Some(v) => println!("pin voltage = {}v", v),
        None    => println!("invalid pin"),
    }
}
