fn main() {
    let s1 = String::from("hello");

    // clone makes a full copy, s1 still valid
    let s2 = s1.clone();

    // both are valid now
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}
