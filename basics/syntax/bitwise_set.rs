fn main() {
    // in embedded you control hardware by setting bits in registers
    // example: GPIO register where each bit = one pin

    let mut register: u8 = 0b0000_0000; // all pins OFF
    println!("start    = {:08b}", register);

    // set bit 0 (pin 0 ON)
    // use OR with 1 shifted to position
    register = register | (1 << 0);
    println!("set bit0 = {:08b}", register); // 0000_0001

    // set bit 3 (pin 3 ON)
    register = register | (1 << 3);
    println!("set bit3 = {:08b}", register); // 0000_1001

    // set bit 7 (pin 7 ON)
    register = register | (1 << 7);
    println!("set bit7 = {:08b}", register); // 1000_1001

    // shorthand using |=
    register |= 1 << 5;
    println!("set bit5 = {:08b}", register); // 1010_1001
}
