fn init_pin(pin: u8) -> Result<&'static str, &'static str> {
    if pin <= 13 {
        Ok("pin initialized")
    } else {
        Err("invalid pin number")
    }
}

fn main() {
    match init_pin(5) {
        Ok(msg)  => println!("ok: {}", msg),
        Err(msg) => println!("error: {}", msg),
    }

    match init_pin(99) {
        Ok(msg)  => println!("ok: {}", msg),
        Err(msg) => println!("error: {}", msg),
    }
}
