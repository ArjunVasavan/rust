fn for_inclusive_range(begin: i32, end: i32) {
    for i in begin..=end {
        println!("{}", i);
    }
}

fn main() {
    for_inclusive_range(1, 5);
}
