import {
  Accessory,
  Categories,
  Characteristic,
  CharacteristicEventTypes,
  Service,
  uuid,
} from "hap-nodejs";
import { trigger } from "./remote";

// optionally set a different storage location with code below. HAPStorage needs to be added to the list of imports above.
// HAPStorage.setCustomStoragePath("...");

const accessoryUuid = uuid.generate("io.morlock.garage");
const accessory = new Accessory("Hoftor", accessoryUuid);

const gateService = new Service.GarageDoorOpener("Hoftor");

let state = false;

// 'On' characteristic is required for the light service
const current = gateService.getCharacteristic(Characteristic.CurrentDoorState)!;
const target = gateService.getCharacteristic(Characteristic.TargetDoorState)!;

function setState(val: boolean) {
  if (val === state) return;

  state = val;
  current.setValue(state);
  target.setValue(state);
  console.log("new state is ", val);

  if (!state) {
    trigger();

    setTimeout(() => {
      setState(true);
    }, 300);
  }
}
current.on(CharacteristicEventTypes.GET, (callback) =>
  callback(undefined, state)
);

target.on(CharacteristicEventTypes.GET, (callback) =>
  callback(undefined, state)
);

target.on(CharacteristicEventTypes.SET, (n, callback) => {
  setState(n == 1);
  callback();
});

accessory.addService(gateService); // adding the service to the accessory

// once everything is set up, we publish the accessory. Publish should always be the last step!
accessory.publish({
  username: "AB:12:99:F4:12:AA",
  pincode: "943-24-157",
  port: 47128,
  category: Categories.GARAGE_DOOR_OPENER, // value here defines the symbol shown in the pairing screen
});

console.log("Accessory setup finished!");
