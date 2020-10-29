/* ================= CONFIG =================*/

// clear out warnings
#![allow(dead_code)]
#![allow(unused_assignments)]

use egg_mode;
use std;
use std::io::{Read, Write};

pub use yansi::Paint;

pub struct Config {
    pub token: egg_mode::Token,
    pub user_id: u64,
    pub screen_name: String,
}

impl Config {
    pub async fn load() -> Self {
        let a1 = Config::load_inner().await;
        if let Some(conf) = a1 {
            return conf;
        }

        Config::load_inner().await.unwrap()
    }

    /// This needs to be a separate function so we can retry after creating the
    /// settings file. Idealy we would recurse, but that requires boxing
    /// the output which doesn't seem worthwhile
    pub async fn load_inner() -> Option<Self> {
        // put keys in file
        let mut consumer_key = String::from("");
        let mut consumer_secret = String::from("");
        let mut c = String::new();
        if let Ok(mut f) = std::fs::File::open("./src/config/keys") {
            f.read_to_string(&mut c).unwrap();
            let mut iter = c.split('\n');

            if let Some(key) = iter.next() {
                consumer_key = key.to_string();
            } else {
                println!("\n[Typically this error indicates incorrect consumer api keys!]");
                println!("Go to ./src/config/keys and fill-in your Twitter developer API-key\nand API-secret-key on the first and second line, respectively.\nThen execute with 'cargo run'.\n");
                std::process::exit(1);
            }

            if let Some(secret) = iter.next() {
                consumer_secret = secret.to_string();
            } else {
                println!("\n[Typically this error indicates incorrect consumer api keys!]");
                println!("Go to ./src/config/keys and fill-in your Twitter developer API-key\nand API-secret-key on the first and second line, respectively.\nThen execute with 'cargo run'.\n");
                std::process::exit(1);
            }
        } else {
            // create file
            let mut f = std::fs::File::create("./src/config/keys").unwrap();
            let create_file = String::from("replace this line with your API-key\nreplace this line with your API-secret-key");
            f.write_all(create_file.as_bytes()).unwrap();
            println!("\nGo to ./src/config/keys and fill-in your Twitter developer API-key\nand API-secret-key on the first and second line, respectively.\nThen execute with 'cargo run'.\n");
            std::process::exit(1);
        }

        let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);

        let mut config = String::new();
        let user_id: u64;
        let username: String;
        let token: egg_mode::Token;

        //look at all this unwrapping! who told you it was my birthday?
        if let Ok(mut f) = std::fs::File::open("./src/config/settings") {
            f.read_to_string(&mut config).unwrap();

            let mut iter = config.split('\n');

            username = iter.next().unwrap().to_string();
            user_id = u64::from_str_radix(&iter.next().unwrap(), 10).unwrap();
            let access_token = egg_mode::KeyPair::new(
                iter.next().unwrap().to_string(),
                iter.next().unwrap().to_string(),
            );
            token = egg_mode::Token::Access {
                consumer: con_token,
                access: access_token,
            };

            if let Err(err) = egg_mode::auth::verify_tokens(&token).await {
                println!("\n[Typically this error indicates incorrect consumer api keys!]");
                println!("Go to ./src/config/keys and fill-in your Twitter developer API-key\nand API-secret-key on the first and second line, respectively.\nThen execute with 'cargo run'.\n");
                println!("ERROR");
                println!("We've hit an error using your old tokens: {:?}", err);
                println!("We'll have to reauthenticate before continuing.");
                std::fs::remove_file("twitter_settings").unwrap();
            } else {
                println!(
                    "              _       _    __           _ __  __               __        _ "
                );
                println!(
                    "   ____ ___  (_)___  (_)  / /__      __(_) /_/ /____  _____   / /___  __(_)"
                );
                println!("  / __ `__ \\/ / __ \\/ /  / __/ | /| / / / __/ __/ _ \\/ ___/  / __/ / / / / ");
                println!(
                    " / / / / / / / / / / /  / /_ | |/ |/ / / /_/ /_/  __/ /     / /_/ /_/ / /  "
                );
                println!("/_/ /_/ /_/_/_/ /_/_/   \\__/ |__/|__/_/\\__/\\__/\\___/_/      \\__/\\__,_/_/   ");

                println!("\n[welcome, @{}, press any key to begin]\n", username);
            }
        } else {
            let request_token = egg_mode::auth::request_token(&con_token, "oob")
                .await
                .unwrap();

            println!("Go to the following URL, sign in, and give me the PIN that comes back:");
            println!("{}", egg_mode::auth::authorize_url(&request_token));

            let mut pin = String::new();
            std::io::stdin().read_line(&mut pin).unwrap();
            println!("");

            let tok_result = egg_mode::auth::access_token(con_token, &request_token, pin)
                .await
                .unwrap();

            token = tok_result.0;
            user_id = tok_result.1;
            username = tok_result.2;

            match token {
                egg_mode::Token::Access {
                    access: ref access_token,
                    ..
                } => {
                    config.push_str(&username);
                    config.push('\n');
                    config.push_str(&format!("{}", user_id));
                    config.push('\n');
                    config.push_str(&access_token.key);
                    config.push('\n');
                    config.push_str(&access_token.secret);
                }
                _ => unreachable!(),
            }

            let mut f = std::fs::File::create("./src/config/settings").unwrap();
            f.write_all(config.as_bytes()).unwrap();

            println!("              _       _    __           _ __  __               __        _ ");
            println!("   ____ ___  (_)___  (_)  / /__      __(_) /_/ /____  _____   / /___  __(_)");
            println!(
                "  / __ `__ \\/ / __ \\/ /  / __/ | /| / / / __/ __/ _ \\/ ___/  / __/ / / / / "
            );
            println!(" / / / / / / / / / / /  / /_ | |/ |/ / / /_/ /_/  __/ /     / /_/ /_/ / /  ");
            println!(
                "/_/ /_/ /_/_/_/ /_/_/   \\__/ |__/|__/_/\\__/\\__/\\___/_/      \\__/\\__,_/_/   "
            );
            println!("\n[welcome, @{}, press any key to begin]\n", username);
        }

        //TODO: Is there a better way to query whether a file exists?
        if std::fs::metadata("./src/config/settings").is_ok() {
            Some(Config {
                token: token,
                user_id: user_id,
                screen_name: username,
            })
        } else {
            None
        }
    }
}
