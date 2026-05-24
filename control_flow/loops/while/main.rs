fn while_loop(limit: i32) {
    let mut count = 0;
    while count < limit {
        println!("count = {}", count);
        count += 1;
    }
}

fn main() {
    while_loop(5);
}
