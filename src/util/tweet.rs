/* ================= TWEET UTILS =================*/

#![allow(dead_code)]

// imports
use crate::config::config;
use crate::util::misc;
use egg_mode::error::Result;

// print single tweet [run with .await]
pub async fn get_tweet(config: &config::Config, tweet_id: u64) -> Result<String> {
    let mut result = String::from("");

    // .await? is crucial here
    let status = egg_mode::tweet::show(tweet_id, &config.token).await?;
    let tweet = &status;
    let (month, day, year) = misc::date_parse(tweet);

    if let Some(ref user) = tweet.user {
        result = result
            + &format!(
                "{} \n[(@{}) posted at {} {}, {}]",
                tweet.text, user.screen_name, month, day, year
            );
    }

    // use match to extract tweet
    Ok(String::from(result))
}

// print timeline with page_size number of tweets
// use await and matching to extract result
pub async fn get_home_timeline(config: &config::Config, page_size: i32) -> Result<Vec<String>> {
    let mut result: Vec<String> = vec![];

    let home = egg_mode::tweet::home_timeline(&config.token).with_page_size(page_size);
    let (_home, feed) = home.start().await?;
    for status in feed.iter() {
        let mut current_tweet = String::from("");

        let tweet = &status;

        let (month, day, year) = misc::date_parse(tweet);
        if let Some(ref user) = tweet.user {
            current_tweet = current_tweet
                + &format!(
                    "{} \n[(@{}) posted at {} {}, {}]\n================================",
                    tweet.text, user.screen_name, month, day, year
                );
        }

        // add tweet to vector
        result.push(String::from(current_tweet));
    }

    Ok(result)
}
