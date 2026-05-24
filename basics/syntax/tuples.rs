fn main() {
    // tuple holds multiple values of different types
    // useful for returning multiple values from function
    let pin_info: (u8, bool, f32) = (13, true, 3.3);
    //             pin  high  volt

    // access by index using .0 .1 .2
    println!("pin  = {}", pin_info.0); // 13
    println!("high = {}", pin_info.1); // true
    println!("volt = {}", pin_info.2); // 3.3

    // destructure tuple into variables
    let (pin, is_high, voltage) = pin_info;
    println!("pin={} high={} volt={}", pin, is_high, voltage);

    // function returning tuple
    fn get_sensor() -> (u8, f32) {
        (5, 25.5) // pin number and temperature
    }

    let (sensor_pin, temp) = get_sensor();
    println!("sensor pin={} temp={}", sensor_pin, temp);

    // empty tuple () means no value - used in embedded for void return
    let nothing: () = ();
    println!("empty tuple = {:?}", nothing);
}
