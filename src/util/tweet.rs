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
    pub screen_name: String,
}
impl Tweet {
    pub fn to_string(self) -> String {
        let mut result = String::from("");
        result = result + &format!("{} \n[(@{})\n", self.text, self.screen_name);
        result
    }
}

// print timeline with page_size number of tweets [use .await]
pub async fn get_home_timeline(config: &config::Config, page_size: i32) -> Result<Vec<Tweet>> {
    let mut result: Vec<Tweet> = vec![];

    let home = egg_mode::tweet::home_timeline(&config.token).with_page_size(page_size);
    let (_home, feed) = home.start().await?;
    for status in feed.iter() {
        //let mut current_tweet = String::from("");

        let tweet = &status;

        //let (month, day, year) = misc::date_parse(tweet);
        if let Some(ref user) = tweet.user {
            /*
            current_tweet = current_tweet
                + &format!(
                    "{} \n[(@{}) posted at {} {}, {}]\n================================",
                    tweet.text, user.screen_name, month, day, year
                );
            */

            let current_tweet = Tweet {
                text: format!("{}", tweet.text),
                screen_name: format!("{}", user.screen_name),
            };

            // add tweet to vector
            result.push(current_tweet);
        }

        // add tweet to vector
        //result.push(String::from(current_tweet));
    }

    // use match to extract timeline
    Ok(result)
}
