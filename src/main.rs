// adapted from https://github.com/jyvet/gate-remote

use std::{thread::sleep, time::Duration};

use rppal::gpio::{Gpio, OutputPin};

const FRAME_SHORT_DELAY: Duration = Duration::from_micros(300); /* 300µs delay for short pulse.          */
const FRAME_LONG_DELAY: Duration = Duration::from_micros(600); /* 600µs delay for long pulse.           */
const BTW_REEMIT_DELAY_MS: Duration = Duration::from_millis(12); /* 12ms delay before sending same signal.*/
const CODE: [bool; 10] = [
    true, false, true, false, true, false, true, false, true, false,
];

fn send_zero(pin: &mut OutputPin) {
    pin.set_low();
    sleep(FRAME_SHORT_DELAY);
    pin.set_high();
    sleep(FRAME_SHORT_DELAY);
    pin.set_low();
    sleep(FRAME_SHORT_DELAY);
}

fn send_one(pin: &mut OutputPin) {
    pin.set_high();
    sleep(FRAME_LONG_DELAY);
    pin.set_low();
    sleep(FRAME_SHORT_DELAY);
}

fn send_frame(pin: &mut OutputPin, nb_emit: u32) {
    for _ in 0..nb_emit {
        /* Send header */
        send_zero(pin);

        /* Send code */
        for bit in CODE {
            if bit {
                send_one(pin);
            } else {
                send_zero(pin);
            }
        }

        /* Send trailer */
        send_zero(pin);
        send_one(pin);

        sleep(BTW_REEMIT_DELAY_MS);

        println!("Sent")
    }
}

fn main() {
    let mut pin = Gpio::new().unwrap().get(23).unwrap().into_output();
    send_frame(&mut pin, 10);
}
