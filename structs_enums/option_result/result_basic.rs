fn main() {
    // Result means Ok(value) or Err(reason)
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");

    // use match to handle both cases
    match success {
        Ok(v)  => println!("success: {}", v),
        Err(e) => println!("error: {}", e),
    }

    match failure {
        Ok(v)  => println!("success: {}", v),
        Err(e) => println!("error: {}", e),
    }
}
