fn main() {
    // Option means value can be Some(value) or None
    let some_val: Option<i32> = Some(42);
    let no_val: Option<i32>   = None;

    // use match to handle both cases
    match some_val {
        Some(v) => println!("got value: {}", v),
        None    => println!("no value"),
    }

    match no_val {
        Some(v) => println!("got value: {}", v),
        None    => println!("no value"),
    }
}
