pub enum Service {
    Tap,
    Nus
}

impl Service {
    pub fn as_uuid(&self) -> u128 {
        match self {
            Service::Tap => 0xc3ff0001_1d8b_40fd_a56f_c7bd5d0f3370,
            Service::Nus => 0x6e400001_b5a3_f393_e0a9_e50e24dcca9e,
        }
    }
}