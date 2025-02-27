use chrono::Local;

async fn on_mouse_mode_change(identifier, mouse_mode) -> Result<(), Error> {
    print!("{} changed to mode {}", identifier, mouse_mode);
}

async fn on_tapped(identifier, tapcode) -> Result<(), Error> {
    print!("{} tapped {}", identifier, tapcode);
}

async fn on_gesture(identifier, gesture) -> Result<(), Error> {
    print!("{} gesture {}", identifier, gesture);
}

async fn on_moused(identifier, vx, vy, isMouse) -> Result<(), Error> {
    print!("{} mouse movement: {}, {}, {}", identifier, vx, vy, isMouse);
}

async fn on_raw_data(identifier, packets) -> Result<(), Error> {
    // for m in packets:
    //     print(f"{m['type']}, {time.time()}, {m['payload']}")
    for packet_item in packets {
        println!("{}, {}, {}", packet_item.type, Local::now(), packet_item.payload);
    }
}

fn main() -> Result<(), Error> {
    let tapClient = tap_sdk::Client::new();

    let handler = tapClient.branch();

    Dispatcher::builder(tapClient, handler).build().dispatch().await;
    // await client.run()
    // print("Connected: {0}".format(client.client.is_connected))

    // await client.register_air_gesture_events(OnGesture)
    // await client.register_tap_events(OnTapped)
    // await client.register_raw_data_events(OnRawData)
    // await client.register_mouse_events(OnMoused)
    // await client.register_air_gesture_state_events(OnMouseModeChange)

    // print("Set Controller Mode for 5 seconds")
    // await client.set_input_mode(TapInputMode("controller"))
    // await asyncio.sleep(5)

    // print("Force Mouse Mode for 5 seconds")
    // await client.set_input_type(InputType.MOUSE)
    // await asyncio.sleep(5)
    
    // print("Force keyboard Mode for 5 seconds")
    // await client.set_input_type(InputType.KEYBOARD)
    // await asyncio.sleep(5)

    // print("Set auto Mode for 10 seconds")
    // await client.set_input_type(InputType.AUTO)
    // await asyncio.sleep(10)

    // print("Set Text Mode for 10 seconds")
    // await client.set_input_mode(TapInputMode("text"))
    // await asyncio.sleep(10)

    // print("Send Haptics")
    // await client.send_vibration_sequence([100, 200, 100, 200, 500])
    // await asyncio.sleep(5)

    // print("Set Raw Mode for 5 seconds")
    // await asyncio.sleep(2)
    // await client.set_input_mode(TapInputMode("raw", sensitivity=[0, 0, 0]))
    // await asyncio.sleep(5)

    Ok(())
}