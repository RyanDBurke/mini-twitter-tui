/* ================= MAIN =================*/

// personal imports
mod config;
mod test;
mod ui;
mod util;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[tokio::main]
async fn main() {
    // use arrows to navigate through tabs
    let mut tab_key: usize = 0;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let config = config::config::Config::load().await;
    let mut key_pressed = false;

    // for tweets
    let max_tweets = 50;
    let timeline = util::tweet::get_home_timeline(&config, max_tweets).await;
    let mut tweets: Vec<util::tweet::Tweet> = vec![];

    // show only 5 tweets at a time, please dont change this
    let mut start: usize = 0;
    let mut end: usize = 5;

    match timeline {
        Ok(t) => {
            tweets = t;
        }
        Err(e) => println!("{}", e),
    }

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                if !key_pressed {
                    key_pressed = true;
                } else {
                    break;
                }
            }
            Key::Esc => {
                if !key_pressed {
                    key_pressed = true;
                } else {
                    break;
                }
            }
            Key::Left => {
                if tab_key != 0 && !key_pressed {
                    tab_key = tab_key - 1;
                } else {
                    key_pressed = true;
                }
            }
            Key::Up => {
                if tab_key != 0 && !key_pressed {
                    tab_key = tab_key - 1;
                } else {
                    key_pressed = true;
                }
            }
            Key::Right => {
                if tab_key != 2 && !key_pressed {
                    tab_key = tab_key + 1;
                } else {
                    key_pressed = true;
                }
            }
            Key::Down => {
                if tab_key != 2 && !key_pressed {
                    tab_key = tab_key + 1;
                } else {
                    key_pressed = true;
                }
            }
            Key::Char('n') => {
                if !key_pressed {
                    key_pressed = true;
                } else {
                    if end + 1 == 51 {
                        start = 0;
                        end = 5;
                    } else {
                        start = start + 1;
                        end  = end + 1;
                    }
                }
            }
            _ => {}
        }

        let user_vec = util::user::get_user(&config, &(config.screen_name));
        let user = util::user::User::build(&config, user_vec).await;

        // split tweets vector to show 5 of the 20 tweets
        let tweet_slice = util::tweet::slice_tweets(&tweets, start, end);
        ui::ui::build_ui(tab_key, user, tweet_slice).expect("UI failed to build.");
        stdout.flush().unwrap();
    }
}
