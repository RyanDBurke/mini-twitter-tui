/* ================= BUILD =================*/

// clear out warnings
#![allow(dead_code)]
#![allow(unused_imports)]

// personal imports
use crate::config;
// mod util;
use crate::util::user_utils::*;
use crate::util::tweet_utils::*;

// 3rd-party library imports
use egg_mode::user;
use egg_mode::error::Result;

use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;


// native library imports
use std::vec::Vec;

#[tokio::main]
pub async fn build_ui() -> Result<()> {     
    let config = config::Config::load().await;

    /*
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    */
    Ok(())
}
