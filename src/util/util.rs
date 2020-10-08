/* ================= UTIL =================*/


/// user and tweets utilities
pub mod user_utils {

    // imports
    use crate::config;
    use egg_mode::user;
    use egg_mode::error::Result;
    use std::vec::Vec;

    // pass a single user, print information
    // execute with .await
    pub async fn print_user(config: &config::Config, users: Vec<egg_mode::user::UserID>) -> Result<()> {
        for user in user::lookup(users, &config.token)
            .await
            .unwrap()
            .response
            .iter()
        {
            println!("");
            println!("{} (@{})", user.name, user.screen_name);
            println!("Created at {}", user.created_at);
            println!(
                "Follows {}, followed by {}",
                user.friends_count, user.followers_count
            );
            if let Some(ref desc) = user.description {
                println!("{}", desc);
            } else {
                println!("[no description provided]");
            }
            
            /*
            // location & link
            match (&user.location, &user.url) {
                (&Some(ref loc), &Some(ref link)) => println!("{} | {}", loc, link),
                (&None, &Some(ref link)) => println!("{}", link),
                (&Some(ref loc), &None) => println!("{}", loc),
                (&None, &None) => (),
            }
            */
        }

        Ok(())
    }

    // returns a single user given
    pub fn get_user(config: &config::Config, screen_name: &String) ->  Vec<egg_mode::user::UserID> { 

        let mut users: Vec<egg_mode::user::UserID> = vec![];
        users.push(config.user_id.into());
        users.push((screen_name.to_string()).into());

        users
    }
}

pub mod tweet_utils {

    // imports
    use crate::config;
    use crate::util::misc_utils::*;
    use egg_mode::user;
    use egg_mode::error::Result;
    use std::vec::Vec;
    use yansi::Paint;
    use std::fmt::Display;
    use chrono::{NaiveDate, Datelike, Weekday};

    // print single tweet [run with .await]
    pub async fn print_tweet(config: &config::Config, tweet_id: u64) -> Result<()> {

        // .await? is crucial here
        let status = egg_mode::tweet::show(tweet_id, &config.token).await?;
        let tweet = &status;
        let (month, day, year) = date_parse(tweet);

        if let Some(ref user) = tweet.user {              
            println!(
                "{} \n[(@{}) posted at {} {}, {}]",
                tweet.text,
                user.screen_name,
                month,
                day,
                year
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

            let (month, day, year) = date_parse(tweet);
            println!("");
            if let Some(ref user) = tweet.user {              
                println!(
                    "{} \n[(@{}) posted at {} {}, {}]",
                    tweet.text,
                    user.screen_name,
                    month,
                    day,
                    year
                );            
            }
        }

        Ok(())
    }
}

pub mod misc_utils {

    use chrono::{NaiveDate, Datelike, Weekday};

    // returns (month, day, year)
    pub fn date_parse(tweet: &egg_mode::tweet::Tweet) -> (&str, u32, i32) {
        let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        (months[tweet.created_at.month0() as usize], tweet.created_at.day(), tweet.created_at.year())
    }
}