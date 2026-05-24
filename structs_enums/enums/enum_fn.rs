enum Led {
    On,
    Off,
}

fn led_status(state: Led) -> &'static str {
    match state {
        Led::On  => "LED is ON",
        Led::Off => "LED is OFF",
    }
}

fn main() {
    println!("{}", led_status(Led::On));
    println!("{}", led_status(Led::Off));
}
