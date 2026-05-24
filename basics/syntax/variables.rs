fn main() {
    // let creates a variable
    // by default variables are immutable (cannot change)
    let x = 5;
    println!("x = {}", x);

    // mut makes variable mutable (can change)
    let mut y = 10;
    println!("y before = {}", y);
    y = 20; // this works because y is mut
    println!("y after  = {}", y);

    // const is always immutable and needs type
    // use for fixed values like pin numbers, baudrate
    const MAX_PIN: u8 = 13;
    const BAUD_RATE: u32 = 9600;
    println!("max pin  = {}", MAX_PIN);
    println!("baud     = {}", BAUD_RATE);

    // let without mut - trying to change will ERROR
    let z = 99;
    // z = 100; // ERROR: cannot assign twice to immutable variable
    println!("z = {}", z);
}
