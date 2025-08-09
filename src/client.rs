use btleplug::api::{BDAddr, Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Manager, Peripheral as PlatformPeripheral};
use tokio::time::sleep;
use std::error::Error;
use std::time::Duration;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::event_manager::EventManager;
use crate::types::input_type::{InputType, InputMode};
use crate::types::mouse_mode::MouseModes;
use crate::types::air_gestures::AirGestures;

// [markdown]
// `Client` class 
#[derive(Debug)]
pub struct Client {
    bluetooth_manager: btleplug::platform::Manager,
    event_manager: Arc<Mutex<EventManager>>,
    address: Option<BDAddr>,
    peripheral: Option<Arc<Mutex<PlatformPeripheral>>>,
    input_mode: Option<InputMode>,
    input_type: Option<InputType>,
    auto_refresh_running: bool,
}

impl Client {
    pub async fn new(address: Option<BDAddr>) -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new().await?;
        let event_manager = Arc::new(Mutex::new(EventManager::new()));
        let mut client = Client {
            bluetooth_manager: manager,
            address: None,
            event_manager,
            peripheral: None,
            input_mode: Some(InputMode::Text),
            input_type: Some(InputType::Auto),
            auto_refresh_running: false,
        };

        let address = if address.is_none() {
            match client.get_mac_address().await {
                Ok(mac_address) => mac_address,
                Err(error) => {
                    panic!("Error during taking Tap device address: {:?}", error);
                }
            }
        } else {
            address.unwrap()
        };

        client.address = Some(address);

        return Ok(client);
    }

    async fn get_mac_address(&self) -> Result<BDAddr, Box<dyn Error>> {
        let adapter_list = self.bluetooth_manager.adapters().await?;
        if adapter_list.is_empty() {
            return Err("No Bluetooth adapters found".into());
        }

        for adapter in adapter_list.iter() {
            println!("Starting scan...");
            adapter
                .start_scan(ScanFilter::default())
                .await
                .expect("Can't scan BLE adapter for connected devices...");

            sleep(Duration::from_secs(2)).await;

            let peripherals = adapter.peripherals().await?;
    
            if peripherals.is_empty() {
                return Err("No Bluetooth devices found".into());
            }

            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from(""));

                if is_connected && local_name.starts_with("Tap") {
                    let is_connected = peripheral.is_connected().await?;
                    let address = peripheral.address();
                    println!("Now connected ({:?}) to peripheral {:?} ({:?})", is_connected, &local_name, address);

                    return Ok(address)
                }
            }
        }

        return Err("No Tap device was found. Make sure the device is connected and its human readable name starts with Tap.".into());
    }

    async fn find_peripheral(&self) -> Option<btleplug::platform::Peripheral> {
        let adapters = self.bluetooth_manager.adapters().await.unwrap();
        let adapter = adapters.into_iter().next().unwrap();

        let peripherals = adapter.peripherals().await.unwrap();
        for peripheral in peripherals {
            if peripheral.address() == self.address.unwrap() {
                return Some(peripheral)
            }
        }

        return None;
    }

    pub async fn run (&mut self) {
        let peripheral = self.find_peripheral().await.unwrap();
        peripheral.connect().await.unwrap();
        peripheral.discover_services().await.unwrap();

        let peripheral_arc = Arc::new(Mutex::new(peripheral));
        self.peripheral = Some(Arc::clone(&peripheral_arc));
        self.event_manager.lock().await.set_peripheral(Arc::clone(&peripheral_arc));

        self.set_initial_state().await;
    }

    async fn set_initial_state(&mut self) {
        println!("Setting initial state: Text mode and Auto type");
        self.set_input_mode(InputMode::Text, None).await;
        self.set_input_type(InputType::Auto).await;
    }

    async fn write_to_characteristic(&self, characteristic_uuid: Uuid, data: &[u8]) -> Result<(), Box<dyn Error>> {
        if let Some(peripheral_arc) = &self.peripheral {
            let characteristics = {
                let peripheral = peripheral_arc.lock().await;
                peripheral.characteristics().clone()
            };
            
            for characteristic in &characteristics {
                if characteristic.uuid == characteristic_uuid {
                    let peripheral = peripheral_arc.lock().await;
                    peripheral.write(characteristic, data, btleplug::api::WriteType::WithResponse).await?;
                    return Ok(());
                }
            }
            Err(format!("Characteristic {:?} not found", characteristic_uuid).into())
        } else {
            Err("No peripheral connected".into())
        }
    }

    pub async fn register_air_gesture_events(&self, listener: fn(Uuid, AirGestures)) {
        self.event_manager.lock().await.register_air_gesture_events(listener).await;
    }
    pub async fn register_tap_events(&self, listener: fn(Uuid, u8)) {
        self.event_manager.lock().await.register_tap_events(listener).await;
    }
    pub async fn register_raw_data_events(&self, listener: fn(Uuid, Vec<u8>)) {
        self.event_manager.lock().await.register_raw_data_events(listener).await;
    }
    pub async fn register_mouse_events(&self, listener: fn(Uuid, i16, i16, bool)) {
        self.event_manager.lock().await.register_mouse_events(listener).await;
    }
    pub async fn register_air_gesture_state_events(&self, listener: fn(Uuid, bool)) {
        self.event_manager.lock().await.register_air_gesture_state_events(listener).await;
    }

    pub async fn set_input_mode(&mut self, mode: InputMode, sensitivity: Option<Vec<u8>>) {
        let tap_mode_uuid = Uuid::from_u128(crate::types::uuid::characteristics::Characteristic::TapMode.as_uuid());
        
        let command = mode.to_command(sensitivity);
        
        println!("Setting input mode to {:?} with command: {:?}", mode, command);
        if let Err(e) = self.write_to_characteristic(tap_mode_uuid, &command).await {
            eprintln!("Failed to set input mode: {:?}", e);
        }

        self.input_mode = Some(mode);
        self.start_auto_refresh().await;
    }

    pub async fn set_input_type(&mut self, input_type: InputType) {
        let tap_mode_uuid = Uuid::from_u128(crate::types::uuid::characteristics::Characteristic::TapMode.as_uuid());
        
        let command = input_type.to_command();
        
        println!("Setting input type to {:?} with command: {:?}", input_type, command);
        if let Err(e) = self.write_to_characteristic(tap_mode_uuid, &command).await {
            eprintln!("Failed to set input type: {:?}", e);
        }

        self.input_type = Some(input_type);
        self.start_auto_refresh().await;
    }

    pub async fn set_mouse_mode(&self, mouse_mode: MouseModes) {
        let tap_mode_uuid = Uuid::from_u128(crate::types::uuid::characteristics::Characteristic::TapMode.as_uuid());
        
        let command = vec![0x03, 0x0e, 0x00, mouse_mode.to_uid()];
        
        println!("Setting mouse mode to {:?} with command: {:?}", mouse_mode, command);
        if let Err(e) = self.write_to_characteristic(tap_mode_uuid, &command).await {
            eprintln!("Failed to set mouse mode: {:?}", e);
        }
    }

    pub async fn send_vibration_sequence(&self, sequence: Vec<u16>) {
        let ui_cmd_uuid = Uuid::from_u128(crate::types::uuid::characteristics::Characteristic::UiCmd.as_uuid());
        
        let mut command = vec![0x00, 0x02];
        for &duration in sequence.iter().take(18) {
            let value = ((duration / 10) as u8).min(255);
            command.push(value);
        }
        
        println!("Sending vibration sequence: {:?} -> command: {:?}", sequence, command);
        if let Err(e) = self.write_to_characteristic(ui_cmd_uuid, &command).await {
            eprintln!("Failed to send vibration sequence: {:?}", e);
        }
    }

    async fn start_auto_refresh(&mut self) {
        if self.auto_refresh_running {
            return;
        }

        self.auto_refresh_running = true;
        println!("Auto-refresh mechanism started (simplified version)");
    }
}
