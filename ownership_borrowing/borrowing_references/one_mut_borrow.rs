fn main() {
    let mut pin = 13;

    let r1 = &mut pin;
    // let r2 = &mut pin; // ERROR: can't borrow twice as mutable

    // only r1 is valid here
    println!("pin = {}", r1);
}
