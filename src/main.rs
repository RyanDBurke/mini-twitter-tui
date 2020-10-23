/* ================= MAIN =================*/

// personal imports
mod config;
mod test;
mod ui;
mod util;

// control imports
use std::io::{ stdin, stdout, Error};
use std::env;
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

    // command args
    let args: Vec<String> = env::args().collect();

    // boolean for producing fake tweets
    let mut produce_fake_tweets = false;
    if args.len() > 1 {
        if &args[1] == "fake" {
            produce_fake_tweets = true;
        }
    }

    // build tweets (whether real of fake)
    let mut tweets: Vec<util::tweet::Tweet>;
    let num_tweets : i32 = 20;
    if produce_fake_tweets {
        // produce num_fake tweets
        tweets = util::tweet::fake_tweets(num_tweets);
    } else {
        // retrieve real tweets
        let timeline = util::tweet::get_home_timeline(&config, num_tweets).await;

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
    }

    // show only 5 tweets at a time, please dont change this
    let mut start: usize = 0;
    let mut end: usize = start + 5;

    // display info screen
    let mut info = false;

    // matrix holding style (keeps track of arrow position)
    let default_style = Style::default().fg(Color::White);
    let selected_style = Style::default().fg(Color::Rgb(29, 161, 242));
    let mut selected: Vec<Vec<Style>> = vec![vec![default_style; 7]; 3];
    let (x_pos, mut y_pos) = (1, 0); // keep track of position
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
                break;
            }
            Key::Esc => {
                break;
            }

            // vertical keys
            Key::Up => {
                if !info {
                    if y_pos != 0 {
                        // unhighlight old positivon
                        selected[x_pos][y_pos] = default_style;
                        y_pos = y_pos - 1;
                        selected[x_pos][y_pos] = selected_style;
                    } else if y_pos == 0 && x_pos == 1 {
                        // simulate pressing p
                        if start != 0 {
                            start = start - 1;
                            end = end - 1;
                        }
                    }
                }
            }

            Key::Down => {
                if !info {
                    // see which column its in
                    if x_pos == 0 {
                        // left column
                        if y_pos != 6 {
                            // unhighlight old positivon
                            selected[x_pos][y_pos] = default_style;
                            y_pos = y_pos + 1;
                            selected[x_pos][y_pos] = selected_style;
                        }
                    } else if x_pos == 1 {
                        // middle column
                        if y_pos != 4 {
                            // unhighlight old positivon
                            selected[x_pos][y_pos] = default_style;
                            y_pos = y_pos + 1;
                            selected[x_pos][y_pos] = selected_style;
                        } else { //  y_pos is 4
                            // simulate pressing n
                            if end == (num_tweets as usize) {
                                start = 0;
                                end = 5;
                            } else {
                                start = start + 1;
                                end = end + 1;
                            }
                        }
                    } else {
                        // right column
                        if y_pos != 4 {
                            // unhighlight old positivon
                            selected[x_pos][y_pos] = default_style;
                            y_pos = y_pos + 1;
                            selected[x_pos][y_pos] = selected_style;
                        }
                    }
                }
            }

            // horizontal keys
            /*
            Key::Left => {
                if !info {
                    if x_pos != 0 {
                        // unhighlight old positivon
                        selected[x_pos][y_pos] = default_style;
                        x_pos = x_pos - 1;
                        y_pos = 0;
                        selected[x_pos][y_pos] = selected_style;
                    }
                }
            }

            Key::Right => {
                if !info {
                    if x_pos != 3 {
                        // unhighlight old positivon
                        selected[x_pos][y_pos] = default_style;
                        x_pos = x_pos + 1;
                        y_pos = 0;
                        selected[x_pos][y_pos] = selected_style;
                    }
                }
            }
            */

            // next tweet
            Key::Char('n') => {
                if !info {
                    if end == 50 {
                        start = 0;
                        end = 5;
                    } else {
                        start = start + 1;
                        end = end + 1;
                    }
                }
            }

            // previous tweet
            Key::Char('p') => {
                if !info {
                    if start != 0 {
                        start = start - 1;
                        end = end - 1;
                    }
                }
            }

            // refresh timeline
            Key::Char('r') => {
                if !info {
                    if  !produce_fake_tweets {
                        let timeline = util::tweet::get_home_timeline(&config, num_tweets).await;

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
                    info = true;
                }
            }

            // back
            Key::Char('b') => {
                info = false;
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
