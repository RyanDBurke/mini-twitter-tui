/* ================= CONFIG =================*/

// clear out warnings
#![allow(dead_code)]

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
        
        
        // keys, find a way to keep these hidden
        let consumer_key = String::from("pZuHdmyRc84cDRDNjlLmbqXH5");
        let consumer_secret = String::from("cyghlODDLxlk7zbYjJEaWpNawBNzP3JcTcDxHGCy2JxNDFbUDD");

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
                println!("We've hit an error using your old tokens: {:?}", err);
                println!("We'll have to reauthenticate before continuing.");
                std::fs::remove_file("twitter_settings").unwrap();
            } else {
                println!("[welcome, @{}, press any key to begin]\n", username);
            }
        } else {
            let request_token = egg_mode::auth::request_token(&con_token, "oob").await.unwrap();

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

            println!("[welcome, @{}, press any key to begin]\n", username);
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