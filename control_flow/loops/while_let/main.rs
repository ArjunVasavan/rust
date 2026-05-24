fn while_let_loop(mut stack: Vec<i32>) {
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn main() {
    while_let_loop(vec![1, 2, 3]);
}
