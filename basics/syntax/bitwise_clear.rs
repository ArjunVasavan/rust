fn main() {
    let mut register: u8 = 0b1111_1111; // all pins ON
    println!("start      = {:08b}", register);

    // clear bit 0 (pin 0 OFF)
    // use AND with NOT of 1 shifted to position
    register = register & !(1 << 0);
    println!("clear bit0 = {:08b}", register); // 1111_1110

    // clear bit 3 (pin 3 OFF)
    register = register & !(1 << 3);
    println!("clear bit3 = {:08b}", register); // 1111_0110

    // clear bit 7 (pin 7 OFF)
    register = register & !(1 << 7);
    println!("clear bit7 = {:08b}", register); // 0111_0110

    // shorthand using &=
    register &= !(1 << 5);
    println!("clear bit5 = {:08b}", register); // 0101_0110
}
