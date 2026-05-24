fn read_sensor(pin: u8) -> Option<f32> {
    if pin <= 13 {
        Some(3.3)
    } else {
        None
    }
}

fn main() {
    match read_sensor(5) {
        Some(v) => println!("sensor value = {}", v),
        None    => println!("invalid pin"),
    }

    match read_sensor(99) {
        Some(v) => println!("sensor value = {}", v),
        None    => println!("invalid pin"),
    }
}
