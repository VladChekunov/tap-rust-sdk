#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AirGestures {
    None = 0,
    General = 1,
    UpOneFinger = 2,
    UpTwoFingers = 3,
    DownOneFinger = 4,
    DownTwoFingers = 5,
    LeftOneFinger = 6,
    LeftTwoFingers = 7,
    RightOneFinger = 8,
    RightTwoFingers = 9,
    Pinch = 10,
    ThumbFinger = 12,
    ThumbMiddle = 14,
    StateOpen = 100,
    StateThumbFinger = 101,
    StateThumbMiddle = 102,
}

impl AirGestures {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => AirGestures::None,
            1 => AirGestures::General,
            2 => AirGestures::UpOneFinger,
            3 => AirGestures::UpTwoFingers,
            4 => AirGestures::DownOneFinger,
            5 => AirGestures::DownTwoFingers,
            6 => AirGestures::LeftOneFinger,
            7 => AirGestures::LeftTwoFingers,
            8 => AirGestures::RightOneFinger,
            9 => AirGestures::RightTwoFingers,
            10 => AirGestures::Pinch,
            12 => AirGestures::ThumbFinger,
            14 => AirGestures::ThumbMiddle,
            100 => AirGestures::StateOpen,
            101 => AirGestures::StateThumbFinger,
            102 => AirGestures::StateThumbMiddle,
            _ => AirGestures::None,
        }
    }
}
