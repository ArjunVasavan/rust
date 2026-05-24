fn for_exclusive_range(begin: i32, end: i32) {
    for i in begin..end {
        println!("{}", i);
    }
}

fn main() {
    for_exclusive_range(1, 5);
}
