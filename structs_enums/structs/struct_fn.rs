struct Point {
    x: i32,
    y: i32,
}

fn print_point(p: Point) {
    println!("x={} y={}", p.x, p.y);
}

fn main() {
    let p = Point { x: 10, y: 20 };
    print_point(p);
}
