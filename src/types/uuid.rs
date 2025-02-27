enum UUID {
    TapService,
    NusService,
    TapDataCharacteristic,
    MouseDataCharacteristic,
    UiCmdCharacteristic,
    AirGestureDataCharacteristic,
    TapModeCharacteristic,
    RawSensorsCharacteristic,
}

impl UUID {
    fn as_str(&self) -> &'static str {
        match self {
            UUID::TapService => "c3ff0001-1d8b-40fd-a56f-c7bd5d0f3370",
            UUID::NusService => "6e400001-b5a3-f393-e0a9-e50e24dcca9e",
            UUID::TapDataCharacteristic => "c3ff0005-1d8b-40fd-a56f-c7bd5d0f3370",
            UUID::MouseDataCharacteristic => "c3ff0006-1d8b-40fd-a56f-c7bd5d0f3370",
            UUID::UiCmdCharacteristic => "c3ff0009-1d8b-40fd-a56f-c7bd5d0f3370",
            UUID::AirGestureDataCharacteristic => "c3ff000a-1d8b-40fd-a56f-c7bd5d0f3370",
            UUID::TapModeCharacteristic => "6e400002-b5a3-f393-e0a9-e50e24dcca9e",
            UUID::RawSensorsCharacteristic => "6e400003-b5a3-f393-e0a9-e50e24dcca9e",
        }
    }
}