import { Gpio } from "onoff";

// Durations (in microseconds)
const SHORT = 370;
const LONG = 620;
const BTW_REEMIT_DELAY_MS = 11380; // In microseconds

const CODE: boolean[] = [
  false,
  true,
  false,
  true,
  false,
  true,
  false,
  true,
  false,
  true,
];

const sleep = (duration: number) =>
  new Promise((resolve) => setTimeout(resolve, duration / 1000));

const sendZero = async (pin: Gpio) => {
  pin.writeSync(0);
  await sleep(LONG);
  pin.writeSync(1);
  await sleep(SHORT);
  pin.writeSync(0);
};

const sendOne = async (pin: Gpio) => {
  pin.writeSync(0);
  await sleep(SHORT);
  pin.writeSync(1);
  await sleep(LONG);
  pin.writeSync(0);
};

const sendFrame = async (pin: Gpio, nbEmit: number) => {
  for (let i = 0; i < nbEmit; i++) {
    // Send header
    await sendZero(pin);
    await sendZero(pin);
    await sendZero(pin);

    // Send code
    for (const bit of CODE) {
      if (bit) {
        await sendOne(pin);
      } else {
        await sendZero(pin);
      }
    }

    await sleep(BTW_REEMIT_DELAY_MS);

    console.log("Sent");
  }
};

export const trigger = async () => {
  const pin = new Gpio(23, "out");
  await sendFrame(pin, 10);
  pin.unexport(); // Cleanup GPIO
};
