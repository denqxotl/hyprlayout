use std::env;
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixStream;

use crate::layout_map;

pub async fn listen_layout_change() -> Result<(), Box<dyn std::error::Error>> {
    let runtime_dir = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR is not set");
    let instance =
        env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("HYPRLAND_INSTANCE_SIGNATURE is not set");

    let socket: PathBuf = [
        runtime_dir.as_str(),
        "hypr",
        instance.as_str(),
        ".socket2.sock",
    ]
    .iter()
    .collect();

    let stream = UnixStream::connect(&socket).await?;
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if let Some(rest) = line.strip_prefix("activelayout>>") {
            if let Some(layout) = rest.split(',').nth(1) {
                println!("{}", layout_map::get_layout_short_name(layout));
            }
        }
    }

    Ok(())
}
