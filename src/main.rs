use std::error::Error;
use std::time::Duration;

use rppal::uart::{Parity, Uart};

fn main() -> Result<(), Box<dyn Error>> {
    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;

    uart.set_read_mode(1, Duration::default())?;

    let mut buffer = [0u8; 1];
    loop {
        if uart.read(&mut buffer)? > 0 {
            let s = std::char::from_u32(buffer[0].into());
            match s {
                None => (),
                Some(c) => {
                    print!("{}", c);
                }
            }
        }
    }
}
