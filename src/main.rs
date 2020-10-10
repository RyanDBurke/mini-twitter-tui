/* ================= MAIN =================*/

// personal imports
mod config;
mod test;
mod ui;
mod util;

// control imports
use std::io::{stdin, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::style::{Color, Style};

#[tokio::main]
async fn main() {


    // build config
    let config = config::config::Config::load().await;

    // std
    let stdin = stdin();
    let _stdout = stdout().into_raw_mode().unwrap();
    
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

    // disregard first key pressed
    let mut key_pressed = false;

    // matrix holding style (keeps track of arrow position)
    let default_style = Style::default().fg(Color::White);
    let selected_style = Style::default().fg(Color::Rgb(29, 161, 242));
    let mut selected : Vec<Vec<Style>> = vec![vec![default_style ; 7] ; 3];
    let (mut x_pos, mut y_pos) = (0, 0); // keep track of position
    selected[x_pos][y_pos] = selected_style; // select first position
    /*
        selected[1][4] = "h"            
        
                  Y
        [] [] [] [ ] [] [] []
     X  [] [] [] [h] [] [] []
        [] [] [] [ ] [] [] []
    */



    // build UI, read keys, and update
    for c in stdin.keys() {
        match c.unwrap() {

            // escape or quit
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

            // vertical keys
            Key::Up => {
                if y_pos != 0 && key_pressed {
                    // unhighlight old positivon
                    selected[x_pos][y_pos] = default_style;
                    y_pos = y_pos - 1;
                    selected[x_pos][y_pos] = selected_style;
                } else {
                    key_pressed = true;
                }
            }

            Key::Down => {
                if y_pos != 7 && key_pressed {
                    // unhighlight old positivon
                    selected[x_pos][y_pos] = default_style;
                    y_pos = y_pos + 1;
                    selected[x_pos][y_pos] = selected_style;
                } else {
                    key_pressed = true;
                }
            }

            // horizontal keys
            Key::Left => {
                
            }

            Key::Right => {
                
            }
            Key::Char('n') => {
                if !key_pressed {
                    key_pressed = true;
                } else {
                    if end == 50 {
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
        ui::ui::build_ui(&selected, user, tweet_slice).expect("UI failed to build.");
        //stdout.flush().unwrap();
    }

}
