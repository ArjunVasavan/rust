fn main() {
    let x = 5;

    // integers are Copy, so x is NOT moved
    let y = x;

    // both x and y are valid
    println!("x = {}", x);
    println!("y = {}", y);

    // String is NOT Copy, integers/bools/floats ARE Copy
}
