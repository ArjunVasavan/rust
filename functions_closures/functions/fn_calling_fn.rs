fn square(x: i32) -> i32 {
    x * x
}

fn sum_of_squares(a: i32, b: i32) -> i32 {
    square(a) + square(b)
}

fn main() {
    println!("{}", sum_of_squares(3, 4)); // 25
}
