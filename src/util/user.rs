/* ================= USER UTILS =================*/

#![allow(dead_code)]
#![allow(unused_imports)]

// imports
use crate::config::config;
use egg_mode::error::Result;
use egg_mode::user;
use std::vec::Vec;

// pass a single user, print informations [execute with .await]
pub async fn user_to_string(config: &config::Config, users: Vec<egg_mode::user::UserID>) -> String {
    let mut user_to_string = String::from("");
    for user in user::lookup(users, &config.token)
        .await
        .unwrap()
        .response
        .iter()
    {
        user_to_string = user_to_string + &format!("\n");
        user_to_string = user_to_string + &format!("{} (@{})\n", user.name, user.screen_name);
        user_to_string = user_to_string + &format!("Created at {}\n", user.created_at);
        user_to_string = user_to_string
            + &format!(
                "Follows {}, followed by {}\n",
                user.friends_count, user.followers_count
            );
        if let Some(ref desc) = user.description {
            user_to_string = user_to_string + &format!("{}\n", desc);
        } else {
            user_to_string = user_to_string + &format!("[no description provided]\n");
        }
    }

    String::from(user_to_string)
}

// returns a single user given
pub fn get_user(config: &config::Config, screen_name: &String) -> Vec<egg_mode::user::UserID> {
    let mut users: Vec<egg_mode::user::UserID> = vec![];
    users.push(config.user_id.into());
    users.push((screen_name.to_string()).into());

    users
}
