fn break_and_continue(begin: i32, end: i32, skip: i32, stop: i32) {
    for i in begin..=end {
        if i == skip { continue; }
        if i == stop { break; }
        println!("{}", i);
    }
}

fn main() {
    break_and_continue(1, 10, 3, 7);
}
