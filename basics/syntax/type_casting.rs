fn main() {
    // as converts one type to another
    let x: i32 = 100;

    // cast i32 to u8
    let y: u8 = x as u8;
    println!("i32 to u8  = {}", y);

    // cast f32 to i32 (cuts off decimal)
    let f: f32 = 3.99;
    let i: i32 = f as i32;
    println!("f32 to i32 = {}", i); // 3 not 4

    // cast u8 to f32 (common in embedded for sensor math)
    let raw: u8  = 200;
    let volt: f32 = raw as f32 * (3.3 / 255.0); // convert ADC to voltage
    println!("adc to volt = {:.2}", volt);

    // cast bool to int
    let b: bool = true;
    let n: i32  = b as i32;
    println!("bool to i32 = {}", n); // 1
}
