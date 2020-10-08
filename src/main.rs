/* ================= MAIN =================*/

/* clear out warnings
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
*/

// personal imports
/*
mod config;
mod util; use util::user_utils::*; use util::tweet_utils::*;
mod ui; use ui::build_ui;
*/

mod ui;
mod util;
mod config;

// theres something about one-line main() functions that I love
fn main() {
    ui::ui::build_ui();
}
