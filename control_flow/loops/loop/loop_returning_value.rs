fn loop_returning_value() -> i32 {
    loop {
        let x = 42;
        break x * 2;
    }
}

fn main() {
    let val = loop_returning_value();
    println!("loop value: {}", val);
}
