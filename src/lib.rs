#[cfg(target_os = "macos")]
mod utils {
    use regex::Regex;
    use std::process::Command;

    pub fn get_raw_output() -> String {
        let output = Command::new("pmset")
            .arg("-g")
            .arg("batt")
            .output()
            .expect("Failed to get battery information");

        String::from_utf8_lossy(&output.stdout).to_string()
    }

    pub fn get_percent(raw_output: &String) -> Option<u8> {
        let regex = Regex::new(r"(\d{2})%").unwrap();
        let matches = regex.captures(raw_output);
        let capture = match matches {
            Some(captures) => captures.get(1),
            None => None,
        };
        let percent_str = match capture {
            Some(capture) => Some(capture.as_str()),
            None => None,
        };
        let percent = match percent_str {
            Some(string) => {
                let string_as_int = string.parse::<u8>();
                match string_as_int.is_ok() {
                    true => Some(string_as_int.unwrap()),
                    false => None,
                }
            }
            None => None,
        };

        percent
    }
}

pub struct BatteryStatus {
    pub raw_output: String,
    pub percent: Option<u8>,
}

impl BatteryStatus {
    pub fn new() -> BatteryStatus {
        let raw_output = utils::get_raw_output();
        let percent = utils::get_percent(&raw_output);
        BatteryStatus {
            raw_output,
            percent,
        }
    }
}
