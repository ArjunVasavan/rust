// trait that returns a value
trait Readable {
    fn read(&self) -> f32;
}

struct TempSensor {
    pin: u8,
}

struct HumiditySensor {
    pin: u8,
}

// both sensors implement same trait
impl Readable for TempSensor {
    fn read(&self) -> f32 {
        36.5 // pretend we read from hardware
    }
}

impl Readable for HumiditySensor {
    fn read(&self) -> f32 {
        80.0 // pretend we read from hardware
    }
}

fn main() {
    let temp = TempSensor { pin: 5 };
    let humi = HumiditySensor { pin: 6 };

    // both use same .read() because of trait
    println!("temp = {}", temp.read());
    println!("humi = {}", humi.read());
}
