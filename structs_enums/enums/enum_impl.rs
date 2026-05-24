enum Motor {
    Forward,
    Backward,
    Stop,
}

impl Motor {
    fn describe(&self) -> &str {
        match self {
            Motor::Forward  => "moving forward",
            Motor::Backward => "moving backward",
            Motor::Stop     => "stopped",
        }
    }
}

fn main() {
    let m = Motor::Forward;
    println!("{}", m.describe());
}
