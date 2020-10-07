/* ================= BUILD =================*/

// clear out warnings
#![allow(dead_code)]
#![allow(unused_imports)]

// personal imports
use crate::config;
// mod util;
use crate::util::tweet_utils::*;
use crate::util::user_utils::*;

// 3rd-party library imports
use egg_mode::error::Result;
use egg_mode::user;

// tui library imports
use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

// native library imports
use std::vec::Vec;

// build our terminal UI
pub fn build_ui() -> std::result::Result<(), io::Error> {
    // clear terminal
    print!("\x1B[2J");

    // create terminal
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // build terminal
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(5), // reduced height of block 1
                    Constraint::Percentage(95),
                ]
                .as_ref(),
            )
            .split(f.size());
        let main_block = Block::default().title("twitter-tui").borders(Borders::ALL);
        f.render_widget(main_block, chunks[0]);

        // sidebar
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .vertical_margin(4)
            .constraints(
                [
                    Constraint::Percentage(10), // reduced height of block 1
                    Constraint::Length(40),
                ]
                .as_ref(),
            )
            .split(f.size());
        let sidebar = Block::default().title("sidebar").borders(Borders::ALL);
        f.render_widget(sidebar, chunks[0]);
    })
}

// return us our relevant Config struct
#[tokio::main]
pub async fn get_config() -> config::Config {
    config::Config::load().await
}
