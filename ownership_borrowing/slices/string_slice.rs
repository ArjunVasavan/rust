fn main() {
    let msg = String::from("hello world");

    // slice part of the string
    // [0..5] means index 0 to 4
    let hello = &msg[0..5];
    let world = &msg[6..11];

    println!("{}", hello); // hello
    println!("{}", world); // world
}
