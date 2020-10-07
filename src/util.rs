/* ================= UTIL =================*/


/// user and tweets utilities
pub mod utils {

    // imports
    use crate::config;
    use egg_mode::user;
    use egg_mode::error::Result;
    use std::vec::Vec;

    // pass a single user, print information
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
                println!("(no description provided)");
            }
            match (&user.location, &user.url) {
                (&Some(ref loc), &Some(ref link)) => println!("{} | {}", loc, link),
                (&None, &Some(ref link)) => println!("{}", link),
                (&Some(ref loc), &None) => println!("{}", loc),
                (&None, &None) => (),
            }
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