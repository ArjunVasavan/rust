fn read_temperature(working: bool) -> Result<f32, &'static str> {
    if working {
        Ok(36.5)             // sensor ok
    } else {
        Err("sensor failed") // sensor broken
    }
}

fn main() {
    // unwrap_or_else runs the closure on error
    let temp = read_temperature(true)
        .unwrap_or_else(|e| {
            println!("error: {}", e);
            0.0 // default value on error
        });
    println!("temp = {}", temp); // 36.5

    let temp = read_temperature(false)
        .unwrap_or_else(|e| {
            println!("error: {}", e);
            0.0 // default value on error
        });
    println!("temp = {}", temp); // 0.0
}
