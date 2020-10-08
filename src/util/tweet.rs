/* ================= TWEET UTILS =================*/

// imports
// use crate::config;
mod config;
// use crate::util::misc_utils::*;
//use super::misc;
mod super;
use chrono::{Datelike, NaiveDate, Weekday};
use egg_mode::error::Result;
use egg_mode::user;
use std::fmt::Display;
use std::vec::Vec;
use yansi::Paint;

// print single tweet [run with .await]
pub async fn print_tweet(config: &config::Config, tweet_id: u64) -> Result<()> {
    // .await? is crucial here
    let status = egg_mode::tweet::show(tweet_id, &config.token).await?;
    let tweet = &status;
    let (month, day, year) = misc::date_parse(tweet);

    if let Some(ref user) = tweet.user {
        println!(
            "{} \n[(@{}) posted at {} {}, {}]",
            tweet.text, user.screen_name, month, day, year
        );
    }

    Ok(())
}

// print timeline with page_size number of tweets
pub async fn print_home_timeline(config: &config::Config, page_size: i32) -> Result<()> {
    let home = egg_mode::tweet::home_timeline(&config.token).with_page_size(page_size);
    let (_home, feed) = home.start().await?;
    for status in feed.iter() {
        let tweet = &status;
        //print_tweet_status(&status);

        let (month, day, year) = misc::date_parse(tweet);
        println!("");
        if let Some(ref user) = tweet.user {
            println!(
                "{} \n[(@{}) posted at {} {}, {}]",
                tweet.text, user.screen_name, month, day, year
            );
        }
    }

    Ok(())
}
