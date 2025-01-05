use color_print::ceprintln;
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
    ceprintln!("<red>{}</>", msg);
    let _ = io::stdin().read(&mut [0u8]).unwrap(); // wait for key press
}
///
////////////////////////////////////////////////////////////////////////

pub struct ActionMap {
    map: HashMap<String, String>,
}

impl ActionMap {
    pub fn new(path: &str) -> Result<ActionMap, ()> {
        let mut json_file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                show_error("\nFailed to open configuration file actions.json!\nMake sure it is in the same folder as the executable.\nIf you don't know what to do, redownload the software from the project page.");
                return Err(());
            }
        };

        let mut buffer = String::new();
        json_file.read_to_string(&mut buffer).unwrap();

        Ok(ActionMap {
            map: match serde_json::from_str(buffer.as_str()) {
                Ok(m) => m,
                Err(_) => {
                    show_error("\nFailed to parse JSON data. There is something wrong with the JSON syntax!");
                    return Err(());
                }
            },
        })
    }

    #[cfg(target_os = "linux")]
    fn numpad_keys(&self, keyname: &str) -> Result<Key, ()> {
        match keyname {
            "numpad_0" => Ok(Key::Other(0xffb0)),
            "numpad_1" => Ok(Key::Other(0xffb1)),
            "numpad_2" => Ok(Key::Other(0xffb2)),
            "numpad_3" => Ok(Key::Other(0xffb3)),
            "numpad_4" => Ok(Key::Other(0xffb4)),
            "numpad_5" => Ok(Key::Other(0xffb5)),
            "numpad_6" => Ok(Key::Other(0xffb6)),
            "numpad_7" => Ok(Key::Other(0xffb7)),
            "numpad_8" => Ok(Key::Other(0xffb8)),
            "numpad_9" => Ok(Key::Other(0xffb9)),
            _ => {
                show_error(
                    format!(
                        "\n{} does not exist!\nPlease refer to the key table on the project page!",
                        keyname
                    )
                    .as_str(),
                );
                Err(())
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn numpad_keys(&self, keyname: &str) -> Result<Key, ()> {
        match keyname {
            "numpad_0" => Ok(Key::Other(0x60)),
            "numpad_1" => Ok(Key::Other(0x61)),
            "numpad_2" => Ok(Key::Other(0x62)),
            "numpad_3" => Ok(Key::Other(0x63)),
            "numpad_4" => Ok(Key::Other(0x64)),
            "numpad_5" => Ok(Key::Other(0x65)),
            "numpad_6" => Ok(Key::Other(0x66)),
            "numpad_7" => Ok(Key::Other(0x67)),
            "numpad_8" => Ok(Key::Other(0x68)),
            "numpad_9" => Ok(Key::Other(0x69)),
            _ => {
                show_error(
                    format!(
                        "\n{} does not exist!\nPlease refer to the key table on the project page!",
                        keyname
                    )
                    .as_str(),
                );
                Err(())
            }
        }
    }

    fn keycode(&self, keyname: &str) -> Result<Key, ()> {
        match keyname {
            "enter" => Ok(Key::Return),
            "up" => Ok(Key::UpArrow),
            "down" => Ok(Key::DownArrow),
            "left" => Ok(Key::LeftArrow),
            "right" => Ok(Key::RightArrow),
            "space" => Ok(Key::Space),
            "f1" => Ok(Key::F1),
            "f2" => Ok(Key::F2),
            "f3" => Ok(Key::F3),
            "f4" => Ok(Key::F4),
            "f5" => Ok(Key::F5),
            "f6" => Ok(Key::F6),
            "f7" => Ok(Key::F7),
            "f8" => Ok(Key::F8),
            "f9" => Ok(Key::F9),
            "f10" => Ok(Key::F10),
            "f11" => Ok(Key::F11),
            "f12" => Ok(Key::F12),
            _ => self.numpad_keys(keyname),
        }
    }

    pub fn get_key_by_action(&self, action: &str) -> Result<Key, ()> {
        let keyname = match self.map.get(action) {
            Some(v) => v,
            None => {
                show_error(format!("\nThe case for <{}> is not covered in actions.json!\nPlease complete the configuration file or redownload the software from the project page.", action).as_str());
                return Err(());
            }
        };
        self.keycode(keyname)
    }
}
