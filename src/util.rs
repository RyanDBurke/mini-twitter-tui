pub mod utils {

    // imports
    use egg_mode::user;

    // pass a single user, print information
    pub fn print_user(user: &user::TwitterUser) {
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
}