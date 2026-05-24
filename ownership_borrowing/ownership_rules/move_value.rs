fn main() {
    // s1 owns the string
    let s1 = String::from("hello");

    // s1 moves to s2, s1 no longer valid
    let s2 = s1;

    // this would ERROR: println!("{}", s1);
    println!("{}", s2); // only s2 is valid now
}
