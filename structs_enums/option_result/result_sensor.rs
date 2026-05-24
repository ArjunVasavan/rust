// returns Ok if init success, Err if failed
fn init_sensor(pin: u8) -> Result<&'static str, &'static str> {
    if pin <= 13 {
        Ok("sensor initialized")  // success
    } else {
        Err("invalid pin number") // failure
    }
}

fn main() {
    // valid pin - success case
    match init_sensor(5) {
        Ok(msg)  => println!("ok: {}", msg),
        Err(msg) => println!("error: {}", msg),
    }

    // invalid pin - error case
    match init_sensor(99) {
        Ok(msg)  => println!("ok: {}", msg),
        Err(msg) => println!("error: {}", msg),
    }
}
