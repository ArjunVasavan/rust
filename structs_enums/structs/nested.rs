struct Pin {
    number: u8,
    is_high: bool,
}

struct Board {
    led: Pin,
    button: Pin,
}

fn main() {
    let board = Board {
        led:    Pin { number: 13, is_high: false },
        button: Pin { number: 2,  is_high: true  },
    };
    println!("led pin={} high={}", board.led.number, board.led.is_high);
    println!("button pin={} high={}", board.button.number, board.button.is_high);
}
