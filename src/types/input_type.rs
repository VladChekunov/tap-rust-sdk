#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputType {
    Mouse = 1,
    Keyboard = 2,
    Auto = 3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Text = 0,
    Controller = 1,
    ControllerText = 3,
    Raw = 10,
}

impl InputType {
    pub fn to_command(&self) -> Vec<u8> {
        match self {
            InputType::Mouse => vec![0x03, 0x0d, 0x00, 0x01],
            InputType::Keyboard => vec![0x03, 0x0d, 0x00, 0x02],
            InputType::Auto => vec![0x03, 0x0d, 0x00, 0x03],
        }
    }
}

impl InputMode {
    pub fn to_command(&self, sensitivity: Option<Vec<u8>>) -> Vec<u8> {
        let mut command = match self {
            InputMode::Text => vec![0x03, 0x0c, 0x00, 0x00],
            InputMode::Controller => vec![0x03, 0x0c, 0x00, 0x01],
            InputMode::ControllerText => vec![0x03, 0x0c, 0x00, 0x03],
            InputMode::Raw => vec![0x03, 0x0c, 0x00, 0x0a],
        };
        
        if let InputMode::Raw = self {
            if let Some(sens) = sensitivity {
                for &s in sens.iter().take(3) {
                    command.push(s);
                }
            }
        }
        
        command
    }
}
