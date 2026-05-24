fn loop_with_step(begin: usize, end: usize, step: usize) {
    for i in (begin..=end).step_by(step) {
        println!("{}", i);
    }
}

fn main() {
    loop_with_step(0, 20, 5);
}
