fn main() {
    // shadowing means reusing same variable name
    // different from mut - can even change type

    let x = 5;
    println!("x = {}", x); // 5

    // shadow x with new value
    let x = x + 10;
    println!("x = {}", x); // 15

    // shadow x with different type
    let x = "now i am a string";
    println!("x = {}", x); // now i am a string

    // common use - convert raw sensor value to processed
    let reading = 180;           // raw ADC value
    println!("raw  = {}", reading);

    let reading = reading as f32 * (3.3 / 255.0); // convert to voltage
    println!("volt = {:.2}", reading);
}
