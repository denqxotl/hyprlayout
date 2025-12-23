use serde::Deserialize;
use std::process::Command;

use crate::layout_map::get_layout_short_name;

#[derive(Debug, Deserialize)]
struct Devices {
    keyboards: Vec<Keyboard>,
}

#[derive(Debug, Deserialize)]
struct Keyboard {
    active_keymap: String,
}

pub fn get_current_layout() -> String {
    let output = Command::new("hyprctl")
        .arg("devices")
        .arg("-j")
        .output()
        .expect("Failed to execute hyprctl");

    let devices: Devices =
        serde_json::from_slice(&output.stdout).expect("Failed to parse hyprctl JSON");

    let layout = devices
        .keyboards
        .first()
        .map(|k| k.active_keymap.clone())
        .unwrap_or_default();

    get_layout_short_name(&layout)
}
