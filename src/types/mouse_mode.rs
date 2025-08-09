#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseModes {
    Stdby = 0,
    AirMouse = 1,
    OpticalOne = 2,
    OpticalTwo = 3,
}

impl MouseModes {
    pub fn to_uid(&self) -> u8 {
        *self as u8
    }
}
