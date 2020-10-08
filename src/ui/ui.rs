/* ================= BUILD =================*/

// clear out warnings
#![allow(unused_imports)]

// personal imports
use crate::config::config;
use crate::util;

// 3rd-party library imports
use egg_mode::error::Result;
use egg_mode::user;

// tui library imports
use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::symbols::{line::VERTICAL, DOT};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Tabs, Widget};
use tui::Terminal;

// native library imports
use std::vec::Vec;

// build our terminal UI
pub async fn build_ui() -> std::result::Result<(), io::Error> {
    // User is a User Struct
    let config = config::Config::load().await;
    let user = util::user::get_user(&config, &(config.screen_name));
    let current_user = util::user::User::build(&config, user).await;

    // clear terminal
    print!("\x1B[2J");

    // create terminal
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // build terminal
    terminal.draw(|f| {
        // header
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
        let user_list = [
            current_user.name,
            format!("@{}", current_user.screen_name),
            current_user.desc,
        ]
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();
        let header = Tabs::new(user_list)
            .block(
                Block::default()
                    .title(" twitter-tui ")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            //.highlight_style(Style::default().fg(Color::Yellow))
            .divider(VERTICAL);
        f.render_widget(header, chunks[0]);

        // tabs
        let chunks = Layout::default()
            //.direction(Direction::Horizontal)
            .direction(Direction::Vertical)
            .horizontal_margin(1)
            .vertical_margin(4)
            .constraints([Constraint::Percentage(7), Constraint::Length(2)].as_ref())
            .split(f.size());
        let tabs_list = ["home", "explore", "profile"]
            .iter()
            .cloned()
            .map(Spans::from)
            .collect();
        let tabs = Tabs::new(tabs_list)
            .block(Block::default().title(" tabs ").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(DOT);
        f.render_widget(tabs, chunks[0]);

        // timeline
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(1)
            .vertical_margin(7)
            .constraints([Constraint::Percentage(50), Constraint::Length(2)].as_ref())
            .split(f.size());
        let body = Block::default().title(" home ").borders(Borders::ALL);
        f.render_widget(body, chunks[0]);
    })
}
