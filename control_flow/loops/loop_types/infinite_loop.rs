fn infinite_loop() {
    loop {
        println!("running..");
    }
}

fn loop_returning_value() -> i32 {
    loop {
        let x = 42;
        break x * 2; /* returns 84 */
    }
}

fn main() {
    println!("output -> {}",loop_returning_value());
}
