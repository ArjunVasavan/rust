fn main() {
    // array has fixed size and all same type
    // very common in embedded for buffers
    let pins: [u8; 5] = [2, 3, 4, 5, 6];
    //          type  size

    // access by index starting at 0
    println!("first pin = {}", pins[0]); // 2
    println!("last pin  = {}", pins[4]); // 6

    // array length
    println!("count = {}", pins.len()); // 5

    // loop over array
    for pin in &pins {
        println!("pin = {}", pin);
    }

    // array with same value repeated
    // [0u8; 8] means eight 0s of type u8
    // common for uart/spi buffers in embedded
    let buffer = [0u8; 8];
    println!("buffer = {:?}", buffer); // [0, 0, 0, 0, 0, 0, 0, 0]

    // mutable array - change values
    let mut readings = [0u8; 4]; // 4 sensor readings
    readings[0] = 10;
    readings[1] = 20;
    readings[2] = 30;
    readings[3] = 40;
    println!("readings = {:?}", readings);

    // 2D array (row, col) - useful for display buffers
    let matrix: [[u8; 3]; 2] = [
        [1, 2, 3], // row 0
        [4, 5, 6], // row 1
    ];
    println!("matrix[0][1] = {}", matrix[0][1]); // 2
    println!("matrix[1][2] = {}", matrix[1][2]); // 6
}
