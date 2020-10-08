/* ================= MAIN =================*/

// personal imports
mod ui;
mod util;
mod config;

fn main() {
    ui::ui::build_ui().expect("User Interace failed to build.");
}
