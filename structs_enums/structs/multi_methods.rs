struct Sensor {
    pin: u8,
    value: f32,
}

impl Sensor {
    fn new(pin: u8) -> Sensor {
        Sensor { pin, value: 0.0 }
    }

    fn read(&mut self, value: f32) {
        self.value = value;
    }

    fn is_above_threshold(&self, threshold: f32) -> bool {
        self.value > threshold
    }

    fn print(&self) {
        println!("pin={} value={}", self.pin, self.value);
    }
}

fn main() {
    let mut sensor = Sensor::new(5);
    sensor.read(2.5);
    sensor.print();
    println!("above 2.0? {}", sensor.is_above_threshold(2.0));
}
