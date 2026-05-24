fn nested_loop_with_label(size: i32) {
    'outer: for x in 0..size {
        for y in 0..size {
            if x == y {
                break 'outer;
            }
            println!("({}, {})", x, y);
        }
    }
}

fn main() {
    nested_loop_with_label(4);
}
