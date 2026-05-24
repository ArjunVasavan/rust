trait Sensor {
    // must implement this
    fn read(&self) -> f32;

    // default method - no need to implement unless you want to override
    fn is_above(&self, threshold: f32) -> bool {
        self.read() > threshold
    }
}

struct Temperature {
    value: f32,
}

impl Sensor for Temperature {
    // only need to implement read()
    fn read(&self) -> f32 {
        self.value
    }
    // is_above() is free from default
}

fn main() {
    let t = Temperature { value: 45.0 };
    println!("reading  = {}", t.read());
    println!("above 40 = {}", t.is_above(40.0)); // uses default method
    println!("above 50 = {}", t.is_above(50.0));
}
