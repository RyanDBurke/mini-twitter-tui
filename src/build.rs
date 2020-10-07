/* ================= BUILD =================*/

// clear out warnings
#![allow(dead_code)]
#![allow(unused_imports)]

// personal imports
use crate::config;
// mod util;
use crate::util::user_utils::*;
use crate::util::tweet_utils::*;

// 3rd-party library imports
use egg_mode::user;
use egg_mode::error::Result;


// native library imports
use std::vec::Vec;

#[tokio::main]
pub async fn build() -> Result<()> {     
    let config = config::Config::load().await;

    
    println!("");
    println!("Loading the user's home timeline:");
    print_timeline(&config, 5).await;



    Ok(())
}
