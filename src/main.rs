/* ================= MAIN =================*/

// personal imports
mod config;
mod test;
mod ui;
mod util;

// control imports
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[tokio::main]
async fn main() {
    // use arrows to navigate through tabs
    let mut tab_key: usize = 0;

    // build config
    let config = config::config::Config::load().await;

    // std
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // for tweets
    let max_tweets = 50;
    let timeline = util::tweet::get_home_timeline(&config, max_tweets).await;
    let mut tweets: Vec<util::tweet::Tweet> = vec![];

    // match with timeline
    match timeline {
        Ok(t) => {
            tweets = t;
        }
        Err(e) => println!("{}", e),
    }

    // show only 5 tweets at a time, please dont change this
    let mut start: usize = 0;
    let mut end: usize = 5;

    // up and down arrows either 0: tabs or i:1-5: tweet[i]
    let mut key_state: usize = 0;

    // disregard first key pressed
    let mut key_pressed = false;

    // build UI, read keys, and update
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
                if tab_key != 0 && key_pressed {
                    tab_key = tab_key - 1;
                } else {
                    key_pressed = true;
                }
            }
            Key::Up => {
                if key_state != 0 && key_pressed {
                    key_state = key_state - 1;
                } else {
                    key_pressed = true;
                }
            }
            Key::Right => {
                if tab_key != 2 && key_pressed {
                    tab_key = tab_key + 1;
                } else {
                    key_pressed = true;
                }
            }
            Key::Down => {
                if key_state != 5 && key_pressed {
                    key_state = key_state + 1;
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

        // needed to build user
        let user_vec = util::user::get_user(&config, &(config.screen_name));
        let user = util::user::User::build(&config, user_vec).await;

        // split tweets vector to show 5 of the 20 tweets
        let tweet_slice = util::tweet::slice_tweets(&tweets, start, end);

        // build UI
        ui::ui::build_ui(tab_key, user, tweet_slice, key_state).expect("UI failed to build.");
        //stdout.flush().unwrap();
    }
}
