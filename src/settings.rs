#[derive(PartialEq, Eq, Debug)]
pub enum OutputMode {
    Binary,
    Color,
}

pub struct Data {
    pub bytes: Vec<u8>,
    pub binary: Vec<bool>,
    pub out_mode: OutputMode,
}

impl Data {
    pub fn from_binary(binary: Vec<bool>) -> Data{
        Data {
            bytes: Vec::new(),
            binary,
            out_mode: OutputMode::Binary,
        }
    }

    pub fn from_color(bytes: Vec<u8>) -> Data {
        Data {
            bytes,
            binary: Vec::new(),
            out_mode: OutputMode::Color,
        }
    } 
}

pub struct Settings {
    pub size: i32,
    pub fps: f64,
    pub width: i32,
    pub height: i32,
}

impl Settings {
    pub fn new(size: i32, fps: i32, width: i32, height: i32,) 
            -> Settings {
            Settings {
                size,
                fps: fps as f64,
                height, 
                width
            }
    }
}