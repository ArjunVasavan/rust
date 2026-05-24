fn main() {
    // println! prints with newline at end
    println!("hello world");

    // print! prints without newline
    print!("hello ");
    print!("world ");
    println!(); // just newline

    // {} is placeholder for variable
    let name = "arjun";
    let age  = 20;
    println!("name = {}", name);
    println!("age  = {}", age);

    // multiple placeholders
    println!("name={} age={}", name, age);

    // named placeholders
    println!("name={name} age={age}");

    // :? is debug print (shows raw value)
    let arr = [1, 2, 3];
    println!("{:?}", arr); // [1, 2, 3]

    // :.2 means 2 decimal places for floats
    let volt: f32 = 3.14159;
    println!("{:.2}", volt); // 3.14

    // :08b prints as binary with 8 digits (useful in embedded)
    let reg: u8 = 0b10110011;
    println!("{:08b}", reg); // 10110011

    // :X prints as hex (very common in embedded)
    let addr: u32 = 0x4002_0000;
    println!("{:X}", addr); // 40020000

    // format! creates a string instead of printing
    let msg = format!("pin {} voltage {:.2}v", 5, 3.3);
    println!("{}", msg);
}
