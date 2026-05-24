fn fill_buffer(buf: &mut [u8], value: u8) {
    // write value into every position in buffer
    for byte in buf.iter_mut() {
        *byte = value;
    }
}

fn main() {
    // buffer like uart tx buffer in embedded
    let mut buffer = [0u8; 8];

    println!("before: {:?}", buffer);

    fill_buffer(&mut buffer, 0xFF);

    println!("after:  {:?}", buffer);
}
