// this simulates how embedded code looks
// without actual hardware

// in real embedded this would be:
// #![no_std]
// #![no_main]
// use panic_halt as _;

struct GpioPin {
    pin: u8,
    is_high: bool,
}

impl GpioPin {
    // create new pin - like gpio.get_pin() in HAL
    fn new(pin: u8) -> Self {
        GpioPin { pin, is_high: false }
    }

    // set pin high - like pin.set_high() in HAL
    fn set_high(&mut self) {
        self.is_high = true;
        println!("pin {} HIGH", self.pin);
    }

    // set pin low - like pin.set_low() in HAL
    fn set_low(&mut self) {
        self.is_high = false;
        println!("pin {} LOW", self.pin);
    }
}

// in real embedded this would be:
// #[entry]
// fn main() -> ! {   // ! means runs forever, never returns
fn main() {
    let mut led = GpioPin::new(13);

    // blink loop - runs forever on real hardware
    for _ in 0..5 {
        led.set_high();
        led.set_low();
    }
}
