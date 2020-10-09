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

pub struct Tweet {
    pub text: String,
    pub full_text: String,
    pub all_text: Vec<String>,
    pub screen_name: String,
    pub id: u64,
}

impl Tweet {
    pub fn to_string(self) -> String {
        let mut result = String::from("");
        result = result + &format!("{} \n[(@{})\n", self.text, self.screen_name);
        result
    }
}

// return timeline with page_size number of tweets [use .await]
pub async fn get_home_timeline(config: &config::Config, page_size: i32) -> Result<Vec<Tweet>> {
    let mut result: Vec<Tweet> = vec![];

    let home = egg_mode::tweet::home_timeline(&config.token).with_page_size(page_size);
    let (_home, feed) = home.start().await?;
    for status in feed.iter() {
        let tweet = &status;

        if let Some(ref user) = tweet.user {

            // limit tweets to 65 characters
            let tweet_char_vec: Vec<char> = tweet.text.chars().collect();
            let mut max: usize = tweet_char_vec.len();

            let mut all_text_vec: Vec<String> = vec![];
            let mut current_string = String::from("");
            let mut str_count = 0;
            let mut i = 0;
            while i <= max {

                if str_count == 65 || i == max {
                    if all_text_vec.len() == 3 {
                        break;
                    }
                    all_text_vec.push(current_string);
                    current_string = String::from("");    
                    str_count = 0;                
                } else {
                    current_string.push(tweet_char_vec[i]);
                }

                str_count = str_count + 1;
                i = i + 1;
            }

            if max > 65 {
                max = 65;

                let current_tweet = Tweet {
                    text: format!(
                        "{}--",
                        (tweet.text).chars().skip(0).take(max).collect::<String>()
                    ),
                    full_text: format!("{}", (tweet.text).chars().skip(max).take(tweet_char_vec.len()).collect::<String>()/*(tweet.text)*/),
                    all_text: all_text_vec,
                    screen_name: format!("{}", user.screen_name),
                    id: tweet.id,
                };

                // add tweet to vector
                result.push(current_tweet);
            } else {
                let current_tweet = Tweet {
                    text: format!(
                        "{}",
                        (tweet.text).chars().skip(0).take(max).collect::<String>()
                    ),
                    full_text: format!("{}", (tweet.text).chars().skip(max).take(tweet_char_vec.len()).collect::<String>()/*(tweet.text)*/),
                    all_text: all_text_vec,
                    screen_name: format!("{}", user.screen_name),
                    id: tweet.id,
                };

                // add tweet to vector
                result.push(current_tweet);
            }
        }
    }

    // use match to extract timeline
    Ok(result)
}

pub fn slice_tweets(tweets: &Vec<Tweet>, start: usize, end: usize) -> Vec<&Tweet> {
    let mut result: Vec<&Tweet> = vec![];

    for t in start..end {
        result.push(&tweets[t]);
    }

    result
}
