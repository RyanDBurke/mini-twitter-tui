/* ================= TWEET UTILS =================*/

#![allow(dead_code)]

// imports
use crate::config::config;
use crate::util::misc;
use egg_mode::error::Result;
use rand::Rng;

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

pub struct Tweet {
    pub text: [String; 2],
    pub screen_name: String,
    pub id: u64,
}

// return timeline with page_size number of tweets [use .await]
pub async fn get_home_timeline(config: &config::Config, page_size: i32) -> Result<Vec<Tweet>> {
    let mut result: Vec<Tweet> = vec![];

    let home = egg_mode::tweet::home_timeline(&config.token).with_page_size(page_size);
    let (_home, feed) = home.start().await?;
    for status in feed.iter() {
        let tweet = &status;

        if let Some(ref user) = tweet.user {
            //let tweet_chars: Vec<char> = tweet.text.chars().collect(); // tweet in vec<char>          

            // build Tweet struct
            let current_tweet = Tweet {
                text: split_tweet(tweet.text.chars().collect()),
                screen_name: format!("{}", user.screen_name),
                id: tweet.id,
            };

            // add tweet to vector
            result.push(current_tweet);
        }
    }

    // use match to extract timeline
    Ok(result)
}

pub fn split_tweet(chars: Vec<char>) -> [String; 2] {
    // stash our tweet-lines
    let mut result: [String; 2] = [String::from(""), String::from("")];

    // total chars
    let total_len: usize = chars.len();

    // max string length
    let max = 49;

    let mut current = String::from(" ");
    let mut current_len = 0;
    let mut pos = 0;
    let mut result_iter = 0;

    while pos < total_len && result_iter < 2 {
        if current_len == max {
            result[result_iter] = current; // push string
            result_iter = result_iter + 1; // update result iter
            current = String::from(" "); // clear current for next iteration
            current_len = 0; // reset curren len
        } else if pos == total_len - 1 {
            current.push(chars[pos]); // push char to current string
            result[result_iter] = current; // push string
            break;
        }
        
        current.push(chars[pos]); // push char to current string
        current_len = current_len + 1;
        pos = pos + 1;
    }

    if result[1].chars().collect::<String>().len() >= 49 {
        result[1] = format!("{}...", result[1]);    
    }

    return result;
}

// takes array of tweets and slices
pub fn slice_tweets(tweets: &Vec<Tweet>, start: usize, end: usize) -> Vec<&Tweet> {
    let mut result: Vec<&Tweet> = vec![];

    for t in start..end {
        result.push(&tweets[t]);
    }

    result
}

// generate fake tweets
pub fn fake_tweets(amt: i32) -> Vec<Tweet> {

    let mut tweets : Vec<Tweet> = vec![];
    let mut rng = rand::thread_rng();

    for i in 0..amt {
        // create tweet
        let tweet = Tweet {
            text: [
                    String::from("Lorem ipsum dolor sit amet,"), 
                    String::from("consectetur adipiscing elit."),
                ],
            screen_name: format!("ryan_{}", i),
            id: rng.gen_range(0, 200),
        };

        // add it to tweets
        tweets.push(tweet);
    }

    tweets
}
