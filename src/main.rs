/* ================= MAIN =================*/

// personal imports
mod ui;
mod util;
mod config;
mod test;

#[tokio::main]
async fn main() {
    let _config = config::config::Config::load().await;
    ui::ui::build_ui().expect("User Interace failed to build.");
}
