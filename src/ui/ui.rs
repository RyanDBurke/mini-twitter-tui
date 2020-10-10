/* ================= BUILD =================*/

// clear out warnings
#![allow(unused_imports)]

// personal imports
use crate::config::config;
use crate::util;

// 3rd-party library imports
use egg_mode::error::Result;
use egg_mode::user;
use term_size;

// tui library imports
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::symbols::{line::VERTICAL, DOT};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Tabs, Widget};
use tui::Terminal;

// native library imports
use std::io;
use std::vec::Vec;

// build our terminal UI
pub fn build_ui(
    selected: &Vec<Vec<Style>>,
    current_user: util::user::User,
    tweets: Vec<&util::tweet::Tweet>,
    info: bool,
) -> std::result::Result<(), io::Error> {
    // clear terminal
    print!("\x1B[2J");

    // create terminal
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // get dimensions of terminal
    let mut terminal_width = 0;
    let mut terminal_height = 0;
    if let Some((w, h)) = term_size::dimensions() {
        terminal_width = w;
        terminal_height = h;
    }

    // trying to avoid the user making the terminal too small
    if terminal_width < 117 || terminal_height < 33 {
        terminal.draw(|f| {
            // error
            let error_color = Color::Rgb(168, 66, 59);
            let chunks = Rect {
                x: 1,
                y: 1,
                width: 40,
                height: 3,
            };
            let error_msg = ["consider making the terminal bigger"]
                .iter()
                .cloned()
                .map(Spans::from)
                .collect();
            let size_error = Tabs::new(error_msg)
                .block(Block::default().title(" error ").borders(Borders::ALL))
                .style(Style::default().fg(error_color))
                .divider(VERTICAL);
            f.render_widget(size_error, chunks);

            // recommended
            let chunks = Rect {
                x: 1,
                y: 4,
                width: 40,
                height: 3,
            };
            let recommended_msg = ["117 x 33 [or greater]"]
                .iter()
                .cloned()
                .map(Spans::from)
                .collect();
            let recommended_size = Tabs::new(recommended_msg)
                .block(
                    Block::default()
                        .title(" recommended dimensions ")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White))
                .divider(VERTICAL);
            f.render_widget(recommended_size, chunks);

            // error controls
            let chunks = Rect {
                x: 1,
                y: 7,
                width: 40,
                height: 3,
            };
            let control_list = ["r: reload", "q: quit"]
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
    // display info screen
    else if info {
        terminal.draw(|f| {
            // info
            let chunks = Rect {
                x: 1,
                y: 1,
                width: 80,
                height: 17,
            };

            let description_list = [
                ListItem::new(" "),
                ListItem::new(
                Text::styled(
                    " twitter-tui is a lightweight in-terminal user-interface for twitter.",
                    Style::default().add_modifier(Modifier::ITALIC),
                )),
                ListItem::new(" "),
                ListItem::new(" Hey, I'm Ryan,"),
                ListItem::new(" "),
                ListItem::new(
                    "   An undergrad computer-science student at University of Maryland.",
                ),
                ListItem::new("   I created this because I read up on the rust programming language,"),
                ListItem::new(
                    "   thought it was cool, and decided to think of an interesting project",
                ),
                ListItem::new("   to help me learn rust. As fun as reading official language"),
                ListItem::new("   documentation is (lol) I prefer just making something. The"),
                ListItem::new("   program doesn't allow any write permissions, but maybe I'll"),
                ListItem::new("   add that in one day :)"),
                ListItem::new(" "),
                ListItem::new(" GitHub: RyanDBurke"),
                ListItem::new(" https://ryandburke.github.io/"),
            ];

            let description = List::new(description_list)
                .block(
                    Block::default()
                        .title(" info ")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White));
            f.render_widget(description, chunks);


            // controls
            let chunks = Rect {
                x: 1,
                y: 18,
                width: 21,
                height: 3,
            };
            let control_list = ["q: quit", "b: back"]
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
    // terminal size looks good! let's build the terminal interface
    else {
        terminal.draw(|f| {
            // entire framebody
            let chunks = Rect {
                x: 1,
                y: 1,
                width: 110,
                height: 28,
            };
            let frame_body = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White));
            f.render_widget(frame_body, chunks);

            // header
            let chunks = Rect {
                x: 1,
                y: 1,
                width: 110,
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

            // sidebar
            let chunks = Rect {
                x: 3,
                y: 4,
                width: 16,
                height: 6,
            };
            let sidebar_items = [
                ListItem::new(Text::styled(" home", selected[0][0])),
                ListItem::new(Text::styled(" profile", selected[0][1])),
                ListItem::new(Text::styled(" settings", selected[0][2])),
            ];
            let sidebar = List::new(sidebar_items)
                .block(Block::default().title(" tabs ").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));
            f.render_widget(sidebar, chunks);

            // trending
            let chunks = Rect {
                x: 3,
                y: 10,
                width: 16,
                height: 18,
            };
            let explore_items = [
                ListItem::new(Text::styled(" #home", selected[0][3])),
                ListItem::new(Text::styled(" #explore", selected[0][4])),
                ListItem::new(Text::styled(" #profile", selected[0][5])),
                ListItem::new(Text::styled(" #settings", selected[0][6])),
            ];
            let explore = List::new(explore_items)
                .block(Block::default().title(" trending ").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));
            f.render_widget(explore, chunks);

            // timeline box
            let chunks = Rect {
                x: 20,
                y: 4,
                width: 60,
                height: 24,
            };
            let timeline = Block::default().title(" timeline ").borders(Borders::ALL);
            f.render_widget(timeline, chunks);

            // fill timeline with tweets
            let mut y_margin = 6; // vertical margin (increment by 4 each iteration)
            for i in 0..(tweets.len()) {
                let tweet = &tweets[i];
                let name = format!(" @{} ", tweet.screen_name);

                // single tweet in timeline
                let chunks = Rect {
                    x: 22,
                    y: y_margin,
                    width: 56,
                    height: 4, //4
                };

                // tweet text
                let tweet_item = [
                    ListItem::new(Text::raw(tweet.text[0].clone())),
                    ListItem::new(Text::raw(tweet.text[1].clone())),
                ];

                // build tweet in feed box and render
                let tweet_text = List::new(tweet_item)
                    .block(Block::default().title(name).borders(Borders::ALL))
                    .style(selected[1][i]);

                f.render_widget(tweet_text, chunks);

                // adjust vertical margin of tweet
                y_margin = y_margin + 4;
            }

            // search
            let chunks = Rect {
                x: 81,
                y: 4,
                width: 28,
                height: 24,
            };
            let search = Block::default().title(" misc ").borders(Borders::ALL);
            f.render_widget(search, chunks);

            // controls
            let chunks = Rect {
                x: 1,
                y: 29,
                width: 66,
                height: 3,
            };
            let control_list = ["n: next tweet", "p: prev tweet", "r: refresh", "q: quit", "i: info"]
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
}
