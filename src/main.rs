// adapted from https://github.com/jyvet/gate-remote

use std::{thread::sleep, time::Duration};

use rppal::gpio::{Gpio, OutputPin};

fn main() {
    let mut pin = Gpio::new().unwrap().get(23).unwrap().into_input();
    loop {
        println!("{:?}", pin.is_high());
        sleep(Duration::from_micros(10));
    }
}
