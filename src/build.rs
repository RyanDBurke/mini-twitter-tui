/* ================= BUILD =================*/

// clear out warnings
#![allow(dead_code)]
#![allow(unused_imports)]

// personal imports
use crate::config;
// mod util;
use crate::util::utils::*;

// 3rd-party library imports
use egg_mode::user;
use egg_mode::error::Result;


// native library imports
use std::vec::Vec;

#[tokio::main]
pub async fn build() -> Result<()> {

     
    let config = config::Config::load().await;
    let users = get_user(&config, &config.screen_name);
    print_user(&config, users); // this wont run
    

/*
    print a tweet
    let config = config::Config::load().await;
    let tweet_id = 766678057788829697;

    println!("");
    println!("Load up an individual tweet:");
    let status = egg_mode::tweet::show(tweet_id, &config.token).await?;
    config::print_tweet(&status);

    println!("");
    println!("Loading the user's home timeline:");
    let home = egg_mode::tweet::home_timeline(&config.token).with_page_size(5);
    let (_home, feed) = home.start().await?;
    for status in feed.iter() {
        config::print_tweet(&status);
        println!("");
    }

    Ok(())
*/
    Ok(())
}
