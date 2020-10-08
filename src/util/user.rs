/* ================= USER UTILS =================*/

#![allow(dead_code)]
#![allow(unused_imports)]

// imports
use crate::config::config;
use egg_mode::error::Result;
use egg_mode::user;
use std::vec::Vec;

// pass a single user, return string of info [execute with .await]
pub async fn user_to_string(config: &config::Config, users: Vec<egg_mode::user::UserID>) -> String {
    let mut result = String::from("");
    for user in user::lookup(users, &config.token)
        .await
        .unwrap()
        .response
        .iter()
    {
        result = result + &format!("\n");
        result = result + &format!("{} (@{})\n", user.name, user.screen_name);
        result = result + &format!("Created at {}\n", user.created_at);
        result = result
            + &format!(
                "Follows {}, followed by {}\n",
                user.friends_count, user.followers_count
            );
        if let Some(ref desc) = user.description {
            result = result + &format!("{}\n", desc);
        } else {
            result = result + &format!("[no description provided]\n");
        }
    }

    String::from(result)
}

// returns a single user given
pub fn get_user(config: &config::Config, screen_name: &String) -> Vec<egg_mode::user::UserID> {
    let mut users: Vec<egg_mode::user::UserID> = vec![];
    users.push(config.user_id.into());
    users.push((screen_name.to_string()).into());

    users
}
