fn main() {
    // pretend this is a real microcontroller register
    // each bit controls something on hardware

    // GPIO direction register
    // bit = 0 means INPUT
    // bit = 1 means OUTPUT
    let mut ddr: u8 = 0b0000_0000; // all pins INPUT by default
    println!("DDR start    = {:08b}", ddr);

    // set pin 5 as OUTPUT (like pinMode(5, OUTPUT))
    ddr |= 1 << 5;
    println!("DDR pin5 out = {:08b}", ddr);

    // set pin 3 as OUTPUT
    ddr |= 1 << 3;
    println!("DDR pin3 out = {:08b}", ddr);

    // GPIO output register
    // bit = 0 means LOW
    // bit = 1 means HIGH
    let mut port: u8 = 0b0000_0000; // all pins LOW
    println!("PORT start   = {:08b}", port);

    // set pin 5 HIGH (like digitalWrite(5, HIGH))
    port |= 1 << 5;
    println!("PORT pin5 hi = {:08b}", port);

    // set pin 3 LOW (like digitalWrite(3, LOW))
    port &= !(1 << 3);
    println!("PORT pin3 lo = {:08b}", port);

    // toggle pin 5 (like led blink)
    port ^= 1 << 5;
    println!("PORT toggle5 = {:08b}", port);
}
