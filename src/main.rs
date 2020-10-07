/* ================= MAIN =================*/

// clear out warnings
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

// personal imports
mod config;
mod util; use util::user_utils::*; use util::tweet_utils::*;
mod build; use build::build_ui;

// theres something about one-line main() functions that I love
fn main() { build_ui(); }
