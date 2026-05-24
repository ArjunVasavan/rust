// two separate traits
trait Readable {
    fn read(&self) -> f32;
}

trait Writable {
    fn write(&mut self, val: f32);
}

struct Register {
    value: f32,
}

// implement both traits for Register
impl Readable for Register {
    fn read(&self) -> f32 {
        self.value
    }
}

impl Writable for Register {
    fn write(&mut self, val: f32) {
        self.value = val;
        println!("wrote {}", val);
    }
}

fn main() {
    let mut reg = Register { value: 0.0 };

    reg.write(3.3);                    // use Writable
    println!("read = {}", reg.read()); // use Readable
}
