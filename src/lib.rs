pub enum BatteryState {
    Discharging,
    Charging,
    ACPower,
    Unknown,
}

#[cfg(target_os = "windows")]
mod utils {}

#[cfg(target_os = "macos")]
mod utils {
    use super::BatteryState;
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
        let regex = Regex::new(r"(\d{1,3})%").unwrap();
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

    pub fn get_status(raw_output: &String) -> BatteryState {
        let state_re = Regex::new(r"(charged|charging|discharging)").unwrap();
        let captures = match state_re.captures(raw_output) {
            Some(captures) => captures.get(1),
            None => None,
        };
        let drawing_str = match captures {
            Some(capture) => Some(capture.as_str()),
            None => None,
        };

        match drawing_str.unwrap_or("None") {
            "charged" => BatteryState::ACPower,
            "charging" => BatteryState::Charging,
            "discharging" => BatteryState::Discharging,
            _ => BatteryState::Unknown,
        }
    }
}

pub struct BatteryStatus {
    pub raw_output: String,
    pub percent: Option<u8>,
    pub status: BatteryState,
}

impl BatteryStatus {
    pub fn new() -> BatteryStatus {
        let raw_output = utils::get_raw_output();
        let percent = utils::get_percent(&raw_output);
        let status = utils::get_status(&raw_output);
        BatteryStatus {
            raw_output,
            percent,
            status,
        }
    }
}
