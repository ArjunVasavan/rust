enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::North;
    match dir {
        Direction::North => println!("going north"),
        Direction::South => println!("going south"),
        Direction::East  => println!("going east"),
        Direction::West  => println!("going west"),
    }
}
