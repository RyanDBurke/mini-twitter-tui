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

    /*
    let config = config::config::Config::load().await;
    let userVec = util::user::get_user(&config, &(config.screen_name));
    let user = util::user::User::build(&config, userVec).await;
    */

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let config = config::config::Config::load().await;
    let mut key_pressed = false;

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
            _ => {}
        }

        let user_vec = util::user::get_user(&config, &(config.screen_name));
        let user = util::user::User::build(&config, user_vec).await;
        let timeline = util::tweet::get_home_timeline(&config, 5).await;
        let mut tweets: Vec<util::tweet::Tweet> = vec![];

        match timeline {
            Ok(t) => {tweets = t;},
            Err(e) => println!("Error extracting timeline tweets {}", e),
        }

        // split tweets vector to show 5 of the 20 tweets

        ui::ui::build_ui(tab_key, user, tweets).expect("UI failed to build.");
        stdout.flush().unwrap();
    }
}
