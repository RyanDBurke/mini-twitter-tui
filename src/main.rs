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
            Key::Char('q') => break,
            Key::Esc => break,
            Key::Left => {
                if tab_key != 0 && !key_pressed {
                    tab_key = tab_key - 1;
                } else {
                    key_pressed = true;
                }
            },
            Key::Up => {
                if tab_key != 0 && !key_pressed {
                    tab_key = tab_key - 1;
                } else {
                    key_pressed = true;
                }
            },
            Key::Right => {
                if tab_key != 2 && !key_pressed {
                    tab_key = tab_key + 1;
                } else {
                    key_pressed = true;
                }
            },
            Key::Down => {
                if tab_key != 2 && !key_pressed {
                    tab_key = tab_key + 1;
                } else {
                    key_pressed = true;
                }
            },
            _ => {}
        }

        let user_vec = util::user::get_user(&config, &(config.screen_name));
        let user = util::user::User::build(&config, user_vec).await;
        ui::ui::build_ui(tab_key, user).expect("UI failed to build.");
        stdout.flush().unwrap();
    }
    /*
    ui::ui::build_ui(tab_key, user)
        .expect("UI failed to build.");
        */
}
