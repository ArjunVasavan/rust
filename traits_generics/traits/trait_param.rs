trait Toggleable {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
}

struct Led {
    pin: u8,
    is_on: bool,
}

// implement Toggleable for Led
impl Toggleable for Led {
    fn turn_on(&mut self) {
        self.is_on = true;
        println!("pin {} ON", self.pin);
    }
    fn turn_off(&mut self) {
        self.is_on = false;
        println!("pin {} OFF", self.pin);
    }
}

// function accepts any type that implements Toggleable
fn blink(device: &mut impl Toggleable) {
    device.turn_on();
    device.turn_off();
}

fn main() {
    let mut led = Led { pin: 13, is_on: false };
    blink(&mut led); // works because Led implements Toggleable
}
