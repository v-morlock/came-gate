// adapted from https://github.com/jyvet/gate-remote

use std::{thread::sleep, time::Duration};

use rppal::gpio::{Gpio, OutputPin};

const SHORT: Duration = Duration::from_micros(370);
const LONG: Duration = Duration::from_micros(620);
const BTW_REEMIT_DELAY_MS: Duration = Duration::from_micros(11_380);
const CODE: [bool; 10] = [
    false, true, false, true, false, true, false, true, false, true,
];

fn send_zero(pin: &mut OutputPin) {
    pin.set_low();
    sleep(LONG);
    pin.set_high();
    sleep(SHORT);
    pin.set_low();
}

fn send_one(pin: &mut OutputPin) {
    pin.set_low();
    sleep(SHORT);
    pin.set_high();
    sleep(LONG);
    pin.set_low();
}

fn send_frame(pin: &mut OutputPin, nb_emit: u32) {
    for _ in 0..nb_emit {
        /* Send header */
        send_zero(pin);
        send_zero(pin);
        send_zero(pin);

        /* Send code */
        for bit in CODE {
            if bit {
                send_one(pin);
            } else {
                send_zero(pin);
            }
        }

        sleep(BTW_REEMIT_DELAY_MS);

        println!("Sent")
    }
}

fn main() {
    let mut pin = Gpio::new().unwrap().get(23).unwrap().into_output();
    send_frame(&mut pin, 10);
}
