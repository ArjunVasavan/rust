// trait is like an interface - defines what a type must do
trait Describable {
    fn describe(&self); // any type using this must implement describe
}

struct Led {
    pin: u8,
}

// implement the trait for Led
impl Describable for Led {
    fn describe(&self) {
        println!("LED on pin {}", self.pin);
    }
}

fn main() {
    let led = Led { pin: 13 };
    led.describe();
}
