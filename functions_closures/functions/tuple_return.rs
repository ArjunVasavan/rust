fn min_max(a: i32, b: i32) -> (i32, i32) {
    (a.min(b), a.max(b))
}

fn main() {
    let (min, max) = min_max(10, 3);
    println!("min={} max={}", min, max);
}
