/* ================= MAIN =================*/

mod config;

use egg_mode::user;

#[tokio::main]
async fn main() {
    
    let config = config::Config::load().await;

    println!("");
    println!("Heterogeneous multi-user lookup:");

    let mut users: Vec<egg_mode::user::UserID> = vec![];
    users.push(config.user_id.into());
    users.push("SwiftOnSecurity".into());

    for user in user::lookup(users, &config.token)
        .await
        .unwrap()
        .response
        .iter()
    {
        print_user(user)
    }
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
