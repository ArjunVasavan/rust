fn get_sensor_value(working: bool) -> Option<f32> {
    if working {
        Some(25.5) // sensor working, return value
    } else {
        None       // sensor broken, return nothing
    }
}

fn main() {
    // unwrap_or gives default value if None
    let temp = get_sensor_value(true).unwrap_or(0.0);
    println!("temp = {}", temp); // 25.5

    let temp = get_sensor_value(false).unwrap_or(0.0);
    println!("temp = {}", temp); // 0.0 (default)
}
