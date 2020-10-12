/* ================= MAIN =================*/

// personal imports
mod config;
mod test;
mod ui;
mod util;

// control imports
use std::io::{stdin, stdout, Error};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::style::{Color, Style};

#[tokio::main]
async fn main() -> std::result::Result<(), Error> {
    // build config
    let config = config::config::Config::load().await;

    // std
    let stdin = stdin();
    let _stdout = stdout().into_raw_mode().unwrap();

    // for tweets
    let max_tweets = 50;
    let timeline = util::tweet::get_home_timeline(&config, max_tweets).await;
    let mut tweets: Vec<util::tweet::Tweet>;

    
    // match with timeline
    match timeline {
        Ok(t) => {
            tweets = t;
        }
        Err(_) => {
            println!("Ran out of API calls lol, chill out for like 3m.");
            return Ok(());
        }
    }
    

    // tweets = util::tweet::fake_tweets(50);

    // show only 5 tweets at a time, please dont change this
    let mut start: usize = 0;
    let mut end: usize = 5;

    // disregard first key pressed
    let mut key_pressed = false;

    // display info screen
    let mut info = false;

    // matrix holding style (keeps track of arrow position)
    let default_style = Style::default().fg(Color::White);
    let selected_style = Style::default().fg(Color::Rgb(29, 161, 242));
    let mut selected: Vec<Vec<Style>> = vec![vec![default_style; 7]; 3];
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
                if !info {
                    if y_pos != 0 && key_pressed {
                        // unhighlight old positivon
                        selected[x_pos][y_pos] = default_style;
                        y_pos = y_pos - 1;
                        selected[x_pos][y_pos] = selected_style;
                    } else {
                        key_pressed = true;
                    }
                }
            }

            Key::Down => {
                if !info {
                    // see which column its in
                    if x_pos == 0 {
                        // left column
                        if y_pos != 6 && key_pressed {
                            // unhighlight old positivon
                            selected[x_pos][y_pos] = default_style;
                            y_pos = y_pos + 1;
                            selected[x_pos][y_pos] = selected_style;
                        } else {
                            key_pressed = true;
                        }
                    } else if x_pos == 1 {
                        // middle column
                        if y_pos != 4 && key_pressed {
                            // unhighlight old positivon
                            selected[x_pos][y_pos] = default_style;
                            y_pos = y_pos + 1;
                            selected[x_pos][y_pos] = selected_style;
                        } else {
                            key_pressed = true;
                        }
                    } else {
                        // right column
                        if y_pos != 4 && key_pressed {
                            // unhighlight old positivon
                            selected[x_pos][y_pos] = default_style;
                            y_pos = y_pos + 1;
                            selected[x_pos][y_pos] = selected_style;
                        } else {
                            key_pressed = true;
                        }
                    }
                }
            }

            // horizontal keys
            Key::Left => {
                if !info {
                    if x_pos != 0 && key_pressed {
                        // unhighlight old positivon
                        selected[x_pos][y_pos] = default_style;
                        x_pos = x_pos - 1;
                        y_pos = 0;
                        selected[x_pos][y_pos] = selected_style;
                    } else {
                        key_pressed = true;
                    }
                }
            }

            Key::Right => {
                if !info {
                    if x_pos != 3 && key_pressed {
                        // unhighlight old positivon
                        selected[x_pos][y_pos] = default_style;
                        x_pos = x_pos + 1;
                        y_pos = 0;
                        selected[x_pos][y_pos] = selected_style;
                    } else {
                        key_pressed = true;
                    }
                }
            }

            // next tweet
            Key::Char('n') => {
                if !info {
                    if !key_pressed {
                        key_pressed = true;
                    } else {
                        if end == 50 {
                            start = 0;
                            end = 5;
                        } else {
                            start = start + 1;
                            end = end + 1;
                        }
                    }
                }
            }

            // previous tweet
            Key::Char('p') => {
                if !info {
                    if !key_pressed {
                        key_pressed = true;
                    } else {
                        if start == 0 {
                            start = 0;
                            end = 5;
                        } else {
                            start = start - 1;
                            end = end - 1;
                        }
                    }
                }
            }

            // refresh timeline
            Key::Char('r') => {
                if !info {
                    if !key_pressed {
                        key_pressed = true;
                    } else {
                        
                        let max_tweets = 50;
                        let timeline = util::tweet::get_home_timeline(&config, max_tweets).await;

                        // match with timeline
                        match timeline {
                            Ok(t) => {
                                tweets = t;
                            }
                            Err(_) => {
                                println!("Ran out of API calls lol, chill out for like 3m.");
                                return Ok(());
                            }
                        }

                        start = 0;
                        end = 5;
                        
                    }
                }
            }

            // info
            Key::Char('i') => {
                if !info {
                    if !key_pressed {
                        key_pressed = true;
                    } else {
                        info = true;
                    }
                }
            }

            // back
            Key::Char('b') => {
                if !key_pressed {
                    key_pressed = true;
                } else {
                    info = false;
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
        ui::ui::build_ui(&selected, user, tweet_slice, info).expect("UI failed to build.");
        //stdout.flush().unwrap();
    }

    Ok(())
}
