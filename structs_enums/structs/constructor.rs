struct Led {
    pin: u8,
    is_on: bool,
}

impl Led {
    fn new(pin: u8) -> Led {
        Led { pin, is_on: false }
    }

    fn turn_on(&mut self) {
        self.is_on = true;
        println!("pin {} ON", self.pin);
    }

    fn turn_off(&mut self) {
        self.is_on = false;
        println!("pin {} OFF", self.pin);
    }
}

fn main() {
    let mut led = Led::new(13);
    led.turn_on();
    led.turn_off();
}
