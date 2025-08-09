use btleplug::api::bleuuid::BleUuid;
use uuid::Uuid;
use tap_sdk::types::input_type::{InputType, InputMode};
use tap_sdk::types::mouse_mode::MouseModes;
use tap_sdk::types::air_gestures::AirGestures;

fn on_gesture(identifier: Uuid, gesture: AirGestures) {
    println!("{} gesture {:?}", identifier.to_short_string(), gesture);
}

fn on_tapped(identifier: Uuid, tapcode: u8) {
    println!("{} tapped {}", identifier.to_short_string(), tapcode);
}

fn on_raw_data(identifier: Uuid, data: Vec<u8>) {
    println!("{} raw data {:?}", identifier.to_short_string(), data);
}

fn on_moused(identifier: Uuid, vx: i16, vy: i16, proximity: bool) {
    println!("{} mouse movement: {}, {}, {}", identifier.to_short_string(), vx, vy, proximity);
}

fn on_mouse_mode_change(identifier: Uuid, in_air_gesture_state: bool) {
    println!("{} air gesture state: {}", identifier.to_short_string(), in_air_gesture_state);
}

#[tokio::main]
async fn main() {
    let mut tap_client = tap_sdk::Client::new(None).await.unwrap();
    tap_client.run().await;

    tap_client.register_air_gesture_events(on_gesture).await;
    tap_client.register_tap_events(on_tapped).await;
    tap_client.register_raw_data_events(on_raw_data).await;
    tap_client.register_mouse_events(on_moused).await;
    tap_client.register_air_gesture_state_events(on_mouse_mode_change).await;

    println!("Setting Controller Mode for 5 seconds...");
    tap_client.set_input_mode(InputMode::Controller, None).await;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("Setting Auto Mode for 5 seconds...");
    tap_client.set_input_type(InputType::Auto).await;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("Setting forced Mouse Mode for 5 seconds...");
    tap_client.set_input_type(InputType::Mouse).await;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    println!("Setting forced Keyboard Mode for 5 seconds...");
    tap_client.set_input_type(InputType::Keyboard).await;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("Setting Text Mode for 5 seconds...");
    tap_client.set_input_mode(InputMode::Text, None).await;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("Setting Controller and Text Mode for 5 seconds...");
    tap_client.set_input_mode(InputMode::ControllerText, None).await;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("Testing vibration feedback...");
    tap_client.send_vibration_sequence(vec![100, 200, 100, 200, 500]).await;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("Returning to Auto Mode for normal operation...");
    tap_client.set_input_type(InputType::Auto).await;
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("Listening for events. Press Ctrl+C to exit.");
    tokio::signal::ctrl_c().await.unwrap();
}