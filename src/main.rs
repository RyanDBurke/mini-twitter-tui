/* ================= MAIN =================*/

mod config;

use egg_mode::user;
use egg_mode::error::Result;

#[tokio::main]
async fn main() -> Result<()> {

    /*    
    let config = config::Config::load().await;

    println!("");
    println!("Heterogeneous multi-user lookup:");

    let mut users: Vec<egg_mode::user::UserID> = vec![];
    users.push(config.user_id.into());
    users.push("_rb70".into());

    for user in user::lookup(users, &config.token)
        .await
        .unwrap()
        .response
        .iter()
    {
        print_user(user)
    }
    */



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
}

fn print_user(user: &user::TwitterUser) {
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
