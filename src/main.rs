use std::{cell::RefCell, rc::Rc};

use hap::{
    accessory::{garage_door_opener, lock, Category, Information},
    characteristic::{Characteristic, Readable, Updatable},
    transport::{IpTransport, Transport},
    Config, HapType,
};

pub struct VirtualDoorInner {
    current_position: u8,
    target_position: u8,
}

#[derive(Clone)]
pub struct VirtualDoor {
    inner: Arc<Mutex<VirtualDoorInner>>,
    current_position: Characteristic<u8>,
}

impl VirtualDoor {
    pub fn new(inner: VirtualDoorInner, current_position: Characteristic<u8>) -> VirtualDoor {
        VirtualDoor {
            inner: Arc::new(Mutex::new(inner)),
            current_position,
        }
    }
}

impl Readable<u8> for VirtualDoor {
    fn on_read(&mut self, hap_type: HapType) -> Option<u8> {
        match hap_type {
            HapType::CurrentPosition => {
                println!("Current position read.");
                Some(self.inner.borrow().current_position)
            }
            HapType::TargetPosition => {
                println!("Target position read.");
                Some(self.inner.borrow().target_position)
            }
            _ => None,
        }
    }
}

impl Updatable<u8> for VirtualDoor {
    fn on_update(&mut self, old_val: &u8, new_val: &u8, hap_type: HapType) {
        match hap_type {
            HapType::CurrentPosition => {
                println!("Current position updated from {} to {}.", old_val, new_val);
                if new_val != old_val {
                    self.inner.borrow_mut().current_position = *new_val;
                }
            }
            HapType::TargetPosition => {
                println!("Target position updated from {} to {}.", old_val, new_val);
                if new_val != old_val {
                    {
                        let mut inner = self.inner.borrow_mut();
                        inner.target_position = *new_val;
                        inner.current_position = *new_val;
                    }
                    self.current_position.set_value(*new_val).unwrap();
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let mut door = door::new(Information {
        name: "Door".into(),
        ..Default::default()
    })
    .unwrap();
    let virtual_door = VirtualDoor::new(
        VirtualDoorInner {
            current_position: 0,
            target_position: 0,
        },
        door.inner.door.inner.current_position.clone(),
    );
    door.inner
        .door
        .inner
        .current_position
        .set_readable(virtual_door.clone())
        .unwrap();
    door.inner
        .door
        .inner
        .current_position
        .set_updatable(virtual_door.clone())
        .unwrap();
    door.inner
        .door
        .inner
        .target_position
        .set_readable(virtual_door.clone())
        .unwrap();
    door.inner
        .door
        .inner
        .target_position
        .set_updatable(virtual_door)
        .unwrap();

    let config = Config {
        name: "Door".into(),
        category: Category::Door,
        ..Default::default()
    };
    let mut ip_transport = IpTransport::new(config).unwrap();
    ip_transport.add_accessory(door).unwrap();
    ip_transport.start().unwrap();
}
