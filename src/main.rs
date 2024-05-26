// adapted from https://github.com/jyvet/gate-remote

use std::{thread::sleep, time::Duration};

use sysfs_gpio::{Direction, Pin};

const FRAME_SHORT_DELAY: Duration = Duration::from_micros(300); /* 300µs delay for short pulse.          */
const FRAME_LONG_DELAY: Duration = Duration::from_micros(600); /* 600µs delay for long pulse.           */
const BTW_REEMIT_DELAY_MS: Duration = Duration::from_millis(12); /* 12ms delay before sending same signal.*/
const CODE: [bool; 10] = [
    false, true, false, true, false, true, false, true, false, true,
];

fn send_zero(pin: Pin) -> Result<(), sysfs_gpio::Error> {
    pin.set_value(0)?;
    sleep(FRAME_SHORT_DELAY);
    pin.set_value(1)?;
    sleep(FRAME_SHORT_DELAY);
    pin.set_value(0)?;
    sleep(FRAME_SHORT_DELAY);
    Ok(())
}

fn send_one(pin: Pin) -> Result<(), sysfs_gpio::Error> {
    pin.set_value(1)?;
    sleep(FRAME_LONG_DELAY);
    pin.set_value(0)?;
    sleep(FRAME_SHORT_DELAY);
    Ok(())
}

fn send_frame(pin: Pin, nb_emit: u32) -> Result<(), sysfs_gpio::Error> {
    for _ in 0..nb_emit {
        /* Send header */
        send_zero(pin)?;

        /* Send code */
        for bit in CODE {
            if bit {
                send_one(pin)?;
            } else {
                send_zero(pin)?;
            }
        }

        /* Send trailer */
        send_zero(pin)?;
        send_one(pin)?;

        sleep(BTW_REEMIT_DELAY_MS);
    }

    Ok(())
}

//#define  SCAN_MODE                /* Uncomment to enter scan mode.         */
fn main() {
    let pin = Pin::new(23);
    pin.with_exported(|| {
        pin.set_direction(Direction::Out).unwrap();
        send_frame(pin, 10)
    })
    .unwrap();
}
