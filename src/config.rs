/* ================= CONFIG =================*/

/*
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
}

impl Config {
    pub fn load() -> Config {

        println!("loading twitter_api keys!");

        let consumer_key = String::from("pZuHdmyRc84cDRDNjlLmbqXH5");
        let consumer_secret = String::from("cyghlODDLxlk7zbYjJEaWpNawBNzP3JcTcDxHGCy2JxNDFbUDD");
        let access_key = String::from("271056234-mLgnT9HFriRgt3gJvZMjNhBbSBZ75roOiwzcBVDW");
        let access_secret = String::from("jhAI9V3BXWx6IcN9UMEmW8KEETxgJZ9IxDjWrmcKj7C2V");

        Config {
            consumer_key,
            consumer_secret,
            access_key,
            access_secret,
        }
    }
}
*/

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//since this is going to get included in examples that might not use everything, clear out warnings
//that are unnecessary
#![allow(dead_code)]

use egg_mode;
use std;
use std::io::{Read, Write};

//This is not an example that can be built with cargo! This is some helper code for the other
//examples so they can load access keys from the same place.

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
    /// twitter_settings file. Idealy we would recurse, but that requires boxing
    /// the output which doesn't seem worthwhile
    async fn load_inner() -> Option<Self> {
        //IMPORTANT: make an app for yourself at apps.twitter.com and get your
        //key/secret into these files; these examples won't work without them
        let consumer_key = String::from("pZuHdmyRc84cDRDNjlLmbqXH5");
        let consumer_secret = String::from("cyghlODDLxlk7zbYjJEaWpNawBNzP3JcTcDxHGCy2JxNDFbUDD");

        let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);

        let mut config = String::new();
        let user_id: u64;
        let username: String;
        let token: egg_mode::Token;

        //look at all this unwrapping! who told you it was my birthday?
        if let Ok(mut f) = std::fs::File::open("twitter_settings") {
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
                println!("Welcome back, {}!\n", username);
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

            let mut f = std::fs::File::create("twitter_settings").unwrap();
            f.write_all(config.as_bytes()).unwrap();

            println!("Welcome, {}, let's get this show on the road!", username);
        }

        //TODO: Is there a better way to query whether a file exists?
        if std::fs::metadata("twitter_settings").is_ok() {
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