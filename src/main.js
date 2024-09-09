"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const hap_nodejs_1 = require("hap-nodejs");
// optionally set a different storage location with code below. HAPStorage needs to be added to the list of imports above.
// HAPStorage.setCustomStoragePath("...");
const accessoryUuid = hap_nodejs_1.uuid.generate("io.morlock.garage");
const accessory = new hap_nodejs_1.Accessory("Example Accessory Name", accessoryUuid);
const lightService = new hap_nodejs_1.Service.GarageDoorOpener("Example Lightbulb");
let state = false;
// 'On' characteristic is required for the light service
const current = lightService.getCharacteristic(hap_nodejs_1.Characteristic.CurrentDoorState);
const target = lightService.getCharacteristic(hap_nodejs_1.Characteristic.TargetDoorState);
function setState(val) {
    if (val === state)
        return;
    state = val;
    current.setValue(state);
    target.setValue(state);
    console.log("new state is ", val);
    if (!state) {
        setTimeout(() => {
            setState(true);
        }, 300);
    }
}
current.on("get" /* CharacteristicEventTypes.GET */, (callback) => callback(undefined, state));
target.on("get" /* CharacteristicEventTypes.GET */, (callback) => callback(undefined, state));
target.on("set" /* CharacteristicEventTypes.SET */, (n, callback) => {
    setState(n == 1);
    callback();
});
accessory.addService(lightService); // adding the service to the accessory
// once everything is set up, we publish the accessory. Publish should always be the last step!
accessory.publish({
    username: "17:51:07:F4:BC:8A",
    pincode: "678-90-876",
    port: 47128,
    category: 5 /* Categories.LIGHTBULB */, // value here defines the symbol shown in the pairing screen
});
console.log("Accessory setup finished!");
