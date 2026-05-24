fn check_voltage(v: f32) -> &'static str {
    if v < 0.0 {
        return "too low";
    }
    if v > 3.3 {
        return "too high";
    }
    "ok"
}

fn main() {
    println!("{}", check_voltage(-1.0));
    println!("{}", check_voltage(5.0));
    println!("{}", check_voltage(3.0));
}
