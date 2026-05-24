fn main() {
    // basic if else
    let pin = 5;
    if pin <= 13 {
        println!("valid pin");
    } else {
        println!("invalid pin");
    }

    // else if for multiple conditions
    let voltage: f32 = 2.5;
    if voltage < 0.0 {
        println!("too low");
    } else if voltage > 3.3 {
        println!("too high");
    } else {
        println!("ok");
    }

    // if as expression (returns value)
    let status = if pin <= 13 { "valid" } else { "invalid" };
    println!("status = {}", status);

    // if with multiple conditions
    let is_output = true;
    let is_high   = true;
    if is_output && is_high {
        println!("pin is output and high");
    }

    // if with or condition
    let mode = 2;
    if mode == 1 || mode == 2 {
        println!("mode 1 or 2");
    }

    // if not condition
    let is_ready = false;
    if !is_ready {
        println!("not ready yet");
    }
}
