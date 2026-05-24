fn main() {
    let register: u8 = 0b1010_1010;
    println!("register = {:08b}", register);

    // check if bit 1 is set
    // AND with 1 shifted to position
    // if result != 0 then bit is set
    let bit1 = (register >> 1) & 1;
    println!("bit 1 = {}", bit1); // 1 (set)

    let bit0 = (register >> 0) & 1;
    println!("bit 0 = {}", bit0); // 0 (not set)

    // check as bool (common in embedded)
    let is_set = (register & (1 << 3)) != 0;
    println!("bit 3 set? = {}", is_set); // true

    let is_set = (register & (1 << 2)) != 0;
    println!("bit 2 set? = {}", is_set); // false

    // read pin state example
    let gpio_input: u8 = 0b0000_1000; // pin 3 is HIGH
    if (gpio_input & (1 << 3)) != 0 {
        println!("pin 3 is HIGH");
    } else {
        println!("pin 3 is LOW");
    }
}
