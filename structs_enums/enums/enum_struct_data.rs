enum Sensor {
    Temperature(f32),
    Humidity(f32),
    Pressure { value: f32, unit: &'static str },
}

fn main() {
    let t = Sensor::Temperature(36.5);
    let h = Sensor::Humidity(80.0);
    let p = Sensor::Pressure { value: 1013.0, unit: "hPa" };

    match t {
        Sensor::Temperature(v)           => println!("temp = {}C", v),
        Sensor::Humidity(v)              => println!("humidity = {}%", v),
        Sensor::Pressure { value, unit } => println!("pressure = {} {}", value, unit),
    }

    match h {
        Sensor::Temperature(v)           => println!("temp = {}C", v),
        Sensor::Humidity(v)              => println!("humidity = {}%", v),
        Sensor::Pressure { value, unit } => println!("pressure = {} {}", value, unit),
    }

    match p {
        Sensor::Temperature(v)           => println!("temp = {}C", v),
        Sensor::Humidity(v)              => println!("humidity = {}%", v),
        Sensor::Pressure { value, unit } => println!("pressure = {} {}", value, unit),
    }
}
