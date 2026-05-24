fn main() {
    let pin = 13;

    // many read borrows are fine
    let r1 = &pin;
    let r2 = &pin;
    let r3 = &pin;

    println!("r1={} r2={} r3={}", r1, r2, r3);
}
