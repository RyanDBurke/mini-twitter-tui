/* ================= MAIN =================*/

// clear out warnings
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

// personal imports
mod config;
mod util; use util::utils::*;
mod build; use build::build;


fn main() {
    build();
}
