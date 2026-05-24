fn variable_range(begin: i32, end: i32) {
    for i in begin..=end {
        println!("{}", i);
    }
}

fn main() {
    variable_range(3, 7);
}
