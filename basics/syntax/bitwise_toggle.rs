fn main() {
    let mut register: u8 = 0b0000_0000;
    println!("start        = {:08b}", register);

    // toggle bit 3 (flip pin 3)
    // use XOR with 1 shifted to position
    register = register ^ (1 << 3);
    println!("toggle bit3  = {:08b}", register); // 0000_1000

    // toggle again - goes back to 0
    register = register ^ (1 << 3);
    println!("toggle again = {:08b}", register); // 0000_0000

    // shorthand using ^=
    register ^= 1 << 3;
    println!("toggle bit3  = {:08b}", register); // 0000_1000

    // toggle LED example - flip every loop
    let mut led: u8 = 0;
    for i in 0..5 {
        led ^= 1 << 0; // toggle bit 0
        println!("loop {} led = {:08b}", i, led);
    }
}
