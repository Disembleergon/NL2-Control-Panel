use enigo::Key;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

/////////////////////////////////////////////////////////////////////////
/// ERROR HANDLING
///
pub fn show_error(msg: &str) {
    eprintln!("{}", msg);
    let _ = io::stdin().read(&mut [0u8]).unwrap(); // wait for key press
}
///
////////////////////////////////////////////////////////////////////////

pub struct ActionMap {
    map: HashMap<String, String>,
}

impl ActionMap {
    pub fn new(path: &str) -> ActionMap {
        let mut json_file = File::open(path).unwrap_or_else(|_|{
            show_error("\nFailed to open configuration file actions.json!\nMake sure it is in the same folder as the executable.\nIf you don't know what to do, redownload the software from the project page.");
            panic!();
        });

        let mut buffer = String::new();
        json_file.read_to_string(&mut buffer).unwrap();

        ActionMap {
            map: serde_json::from_str(buffer.as_str()).unwrap_or_else(|_| {
                show_error(
                    "\nFailed to parse JSON data. There is something wrong with the JSON syntax!",
                );
                panic!();
            }),
        }
    }

    #[cfg(target_os = "linux")]
    fn numpad_keys(&self, keyname: &str) -> Key {
        match keyname {
            "numpad_0" => Key::Other(0xffb0),
            "numpad_1" => Key::Other(0xffb1),
            "numpad_2" => Key::Other(0xffb2),
            "numpad_3" => Key::Other(0xffb3),
            "numpad_4" => Key::Other(0xffb4),
            "numpad_5" => Key::Other(0xffb5),
            "numpad_6" => Key::Other(0xffb6),
            "numpad_7" => Key::Other(0xffb7),
            "numpad_8" => Key::Other(0xffb8),
            "numpad_9" => Key::Other(0xffb9),
            _ => {
                show_error(
                    format!(
                        "\n{} does not exist!\nPlease refer to the key table on the project page!",
                        keyname
                    )
                    .as_str(),
                );
                panic!();
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn numpad_keys(&self, keyname: &str) -> Key {
        match keyname {
            "numpad_0" => Key::Other(0x60),
            "numpad_1" => Key::Other(0x61),
            "numpad_2" => Key::Other(0x62),
            "numpad_3" => Key::Other(0x63),
            "numpad_4" => Key::Other(0x64),
            "numpad_5" => Key::Other(0x65),
            "numpad_6" => Key::Other(0x66),
            "numpad_7" => Key::Other(0x67),
            "numpad_8" => Key::Other(0x68),
            "numpad_9" => Key::Other(0x69),
            _ => {
                show_error(
                    format!(
                        "\n{} does not exist!\nPlease refer to the key table on the project page!",
                        keyname
                    )
                    .as_str(),
                );
                panic!();
            }
        }
    }

    fn keycode(&self, keyname: &str) -> Key {
        match keyname {
            "enter" => Key::Return,
            "up" => Key::UpArrow,
            "down" => Key::DownArrow,
            "left" => Key::LeftArrow,
            "right" => Key::RightArrow,
            "space" => Key::Space,
            "f1" => Key::F1,
            "f2" => Key::F2,
            "f3" => Key::F3,
            "f4" => Key::F4,
            "f5" => Key::F5,
            "f6" => Key::F6,
            "f7" => Key::F7,
            "f8" => Key::F8,
            "f9" => Key::F9,
            "f10" => Key::F10,
            "f11" => Key::F11,
            "f12" => Key::F12,
            _ => self.numpad_keys(keyname),
        }
    }

    pub fn get_key_by_action(&self, action: &str) -> Key {
        let keyname = match self.map.get(action) {
            Some(v) => v,
            None => {
                show_error(format!("\nThe case for <{}> is not covered in actions.json!\nPlease complete the configuration file or redownload the software from the project page.", action).as_str());
                panic!();
            }
        };
        self.keycode(keyname)
    }
}
