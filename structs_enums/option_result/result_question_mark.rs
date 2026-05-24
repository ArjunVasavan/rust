// ? means: if Ok continue, if Err return Err immediately
fn init_pin(pin: u8) -> Result<&'static str, &'static str> {
    if pin <= 13 {
        Ok("pin ready")
    } else {
        Err("bad pin")
    }
}

fn setup() -> Result<&'static str, &'static str> {
    // ? automatically returns Err if init_pin fails
    // no need to write match every time
    let status = init_pin(5)?;
    println!("init: {}", status);
    Ok("setup done")
}

fn main() {
    match setup() {
        Ok(msg)  => println!("ok: {}", msg),
        Err(msg) => println!("error: {}", msg),
    }
}
