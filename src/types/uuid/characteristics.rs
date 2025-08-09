use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use uuid::Uuid;
use std::error::Error;

#[derive(EnumIter, Debug)]
pub enum Characteristic {
    TapData,
    MouseData,
    AirGestureData,
    UiCmd,
    TapMode,
    RawSensors,
}

impl Characteristic {
    pub fn as_uuid(&self) -> u128 {
        match self {
            Characteristic::TapData => 0xc3ff0005_1d8b_40fd_a56f_c7bd5d0f3370,
            Characteristic::MouseData => 0xc3ff0006_1d8b_40fd_a56f_c7bd5d0f3370,
            Characteristic::AirGestureData => 0xc3ff000a_1d8b_40fd_a56f_c7bd5d0f3370,
            Characteristic::UiCmd => 0xc3ff0009_1d8b_40fd_a56f_c7bd5d0f3370,
            Characteristic::TapMode => 0x6e400002_b5a3_f393_e0a9_e50e24dcca9e,
            Characteristic::RawSensors => 0x6e400003_b5a3_f393_e0a9_e50e24dcca9e,
        }
    }

    pub fn is_tap(uuid: Uuid) -> bool {
        Characteristic::iter()
        .any(
            |tap_uuid| tap_uuid.as_uuid() == uuid.as_u128()
        )
    }

    pub fn from_uuid(uuid: Uuid) -> Result<Characteristic, Box<dyn Error>> {
        Characteristic::iter()
        .find(
            |tap_uuid| tap_uuid.as_uuid() == uuid.as_u128()
        )
        .ok_or_else(|| "No matching characteristic found.".into())
    }
}
