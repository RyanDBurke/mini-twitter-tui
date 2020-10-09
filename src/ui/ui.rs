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
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::symbols::{line::VERTICAL, DOT};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, List, ListItem, Tabs, Widget};
use tui::Terminal;

// native library imports
use std::io;
use std::vec::Vec;

// build our terminal UI
pub fn build_ui(
    tab_key: usize,
    current_user: util::user::User,
    tweets: Vec<&util::tweet::Tweet>,
    key_state: usize,
) -> std::result::Result<(), io::Error> {
    // clear terminal
    print!("\x1B[2J");

    // create terminal
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // build terminal
    terminal.draw(|f| {

        /*=== header ===*/
        let chunks = Rect {
            x: 1,
            y: 1,
            width: 80,
            height: 3,
        };
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
            .divider(VERTICAL);
        f.render_widget(header, chunks);

        /*=== tabs ===*/
        let chunks = Rect {
            x: 1,
            y: 4,
            width: 80,
            height: 3,
        };
        let tabs_list = ["home", "explore", "profile"]
            .iter()
            .cloned()
            .map(Spans::from)
            .collect();
        let tabs = Tabs::new(tabs_list)
            .block(Block::default().title(" tabs ").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Cyan))
            .select(tab_key) // chooses which tab is selected
            .divider(DOT);
        f.render_widget(tabs, chunks);

        /*=== tweet(s) ===*/
        let mut v_margin = 8;
        for i in 0..(tweets.len()) {
            let tweet = &tweets[i];
            let text = format!(" {} ", tweet.text);
            let full_text = format!(" {} ", tweet.full_text);
            let name = format!(" @{} ", tweet.screen_name);
            let mut color = Color::White;
            if i + 1 == key_state {
                color = Color::Cyan;
                let h = 3*tweet.all_text.len() as u16;
                let chunks = Rect {
                    x: 82,
                    y: v_margin,
                    width: 70,
                    height: h,
                };            
                
                // let text_tab = vec![Spans::from(full_text)];/*.iter().cloned();.map(Spans::from).collect();*/
                //let text_tab = [ListItem::new(text.clone()), ListItem::new(full_text.clone())];
                let text_len = tweet.all_text.len();
                let mut text_tab = [ListItem; text_len];

                for j in 0..text_len {
                    text_tab[j] = ListItem::new(tweet.all_text[j].clone());               
                }

                let tweet_text = List::new(text_tab).block(
                    Block::default()
                        .title(name.clone())
                        .borders(Borders::ALL)
                        .style(Style::default().fg(color)),
                );
                f.render_widget(tweet_text, chunks);

            }

            /*=== single tweet ===*/
            let chunks = Rect {
                x: 4,
                y: v_margin,
                width: 74,
                height: 3,
            };            
            
            let text_tab = [Span::raw(text)].iter().cloned().map(Spans::from).collect();
            let tweet_text = Tabs::new(text_tab).block(
                Block::default()
                    .title(name)
                    .borders(Borders::ALL)
                    .style(Style::default().fg(color)),
            );
            f.render_widget(tweet_text, chunks);

            v_margin = v_margin + 4;
        }

        /*=== timeline ===*/
        let chunks = Rect {
            x: 1,
            y: 7,
            width: 80,
            height: 50,
        };
        let tabs_list = ["home", "explore", "profile"]; // redefining tabs_list
        let title = format!(" {} ", tabs_list[tab_key]);
        let body = Block::default().title(title).borders(Borders::ALL);
        f.render_widget(body, chunks);

        /*=== controls === */
        let chunks = Rect {
            x: 1,
            y: 57,
            width: 80,
            height: 3,
        };
        let control_list = ["n: next tweet", "q: quit", "i: info"]
            .iter()
            .cloned()
            .map(Spans::from)
            .collect();
        let controls = Tabs::new(control_list)
            .block(
                Block::default()
                    .title(" controls ")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White)),
            )
            .divider(VERTICAL);
        f.render_widget(controls, chunks);

    })
}
