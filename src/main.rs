/* ================= MAIN =================*/

// personal imports
mod ui;
mod util;
mod config;

fn main() {
    let _config = util::misc::get_config();
    ui::ui::build_ui().expect("User Interace failed to build.");
}
