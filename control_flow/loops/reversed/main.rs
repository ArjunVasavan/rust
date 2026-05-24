fn loop_reversed(begin: i32, end: i32) {
    for i in (begin..=end).rev() {
        println!("{}", i);
    }
}

fn main() {
    loop_reversed(1, 5);
}
