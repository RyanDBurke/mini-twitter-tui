/* ================= MAIN =================*/

// personal imports
mod ui;
mod util;
mod config;
mod test;

#[tokio::main]
async fn main() {
    ui::ui::build_ui().await;
}
