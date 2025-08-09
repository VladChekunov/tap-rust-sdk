use btleplug::platform::Peripheral as PlatformPeripheral;
use uuid::Uuid;
use crate::types::air_gestures::AirGestures;
use crate::types::uuid::characteristics::Characteristic;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::StreamExt;
use btleplug::api::Peripheral;

#[derive(Clone, Debug)]
pub struct EventManager {
    peripheral: Option<Arc<Mutex<PlatformPeripheral>>>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager { peripheral: None }
    }

    pub fn set_peripheral(&mut self, peripheral: Arc<Mutex<PlatformPeripheral>>) {
        self.peripheral = Some(peripheral);
    }

    async fn subscribe_to_characteristic<F>(&self, characteristic_uuid: Uuid, callback: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(Uuid, Vec<u8>) + Send + 'static,
    {
        if let Some(peripheral_arc) = &self.peripheral {
            let characteristics = {
                let peripheral = peripheral_arc.lock().await;
                peripheral.characteristics().clone()
            };
            for characteristic in &characteristics {
                if characteristic.uuid == characteristic_uuid {
                    if !characteristic.properties.contains(btleplug::api::CharPropFlags::NOTIFY) {
                        return Err(format!("Characteristic {:?} does not support notifications", characteristic_uuid).into());
                    }
                    {
                        let peripheral = peripheral_arc.lock().await;
                        peripheral.subscribe(characteristic).await?;
                    }
                    let peripheral_clone = Arc::clone(peripheral_arc);
                    let char_uuid = characteristic.uuid;
                    tokio::spawn(async move {
                        let stream = {
                            let peripheral = peripheral_clone.lock().await;
                            peripheral.notifications().await
                        };
                        if let Ok(mut stream) = stream {
                            println!("Successfully subscribed to characteristic {:?}", char_uuid);
                            while let Some(notification) = stream.next().await {
                                if notification.uuid == char_uuid {
                                    callback(char_uuid, notification.value);
                                }
                            }
                        } else {
                            eprintln!("Failed to get notification stream for characteristic {:?}", char_uuid);
                        }
                    });
                    return Ok(());
                }
            }
            Err(format!("Characteristic {:?} not found", characteristic_uuid).into())
        } else {
            Err("No peripheral connected".into())
        }
    }

    pub async fn register_air_gesture_events(&self, listener: fn(Uuid, AirGestures)) {
        let air_gesture_uuid = Uuid::from_u128(Characteristic::AirGestureData.as_uuid());
        let callback = move |uuid: Uuid, data: Vec<u8>| {
            if data.len() > 0 {
                let gesture_value = data[0] as u8;
                let gesture = AirGestures::from_u8(gesture_value);
                listener(uuid, gesture);
            }
        };
        if let Err(e) = self.subscribe_to_characteristic(air_gesture_uuid, callback).await {
            eprintln!("Failed to register air gesture events: {:?}", e);
        }
    }

    pub async fn register_tap_events(&self, listener: fn(Uuid, u8)) {
        let tap_data_uuid = Uuid::from_u128(Characteristic::TapData.as_uuid());
        let callback = move |uuid: Uuid, data: Vec<u8>| {
            if data.len() > 0 {
                let tapcode = data[0];
                listener(uuid, tapcode);
            }
        };
        if let Err(e) = self.subscribe_to_characteristic(tap_data_uuid, callback).await {
            eprintln!("Failed to register tap events: {:?}", e);
        }
    }

    pub async fn register_raw_data_events(&self, listener: fn(Uuid, Vec<u8>)) {
        let raw_sensors_uuid = Uuid::from_u128(Characteristic::RawSensors.as_uuid());
        let callback = move |uuid: Uuid, data: Vec<u8>| {
            listener(uuid, data);
        };
        if let Err(e) = self.subscribe_to_characteristic(raw_sensors_uuid, callback).await {
            eprintln!("Failed to register raw data events: {:?}", e);
        }
    }

    pub async fn register_mouse_events(&self, listener: fn(Uuid, i16, i16, bool)) {
        let mouse_data_uuid = Uuid::from_u128(Characteristic::MouseData.as_uuid());
        let callback = move |uuid: Uuid, data: Vec<u8>| {
            if data.len() >= 4 {
                let vx = ((data[0] as i16) << 8) | (data[1] as i16);
                let vy = ((data[2] as i16) << 8) | (data[3] as i16);
                let proximity = data.len() > 4 && data[4] != 0;
                listener(uuid, vx, vy, proximity);
            }
        };
        if let Err(e) = self.subscribe_to_characteristic(mouse_data_uuid, callback).await {
            eprintln!("Failed to register mouse events: {:?}", e);
        }
    }

    pub async fn register_air_gesture_state_events(&self, listener: fn(Uuid, bool)) {
        let air_gesture_uuid = Uuid::from_u128(Characteristic::AirGestureData.as_uuid());
        let callback = move |uuid: Uuid, data: Vec<u8>| {
            if data.len() > 0 {
                let in_air_gesture_state = data[0] == 0x14 && data.len() > 1 && data[1] != 0;
                listener(uuid, in_air_gesture_state);
            }
        };
        if let Err(e) = self.subscribe_to_characteristic(air_gesture_uuid, callback).await {
            eprintln!("Failed to register air gesture state events: {:?}", e);
        }
    }
}
