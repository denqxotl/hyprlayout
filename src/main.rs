mod current_layout;
mod layout_listener;
mod layout_map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", current_layout::get_current_layout());
    layout_listener::listen_layout_change().await?;
    Ok(())
}
