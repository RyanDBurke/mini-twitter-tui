/* ================= MISC UTIL =================*/

#![allow(dead_code)]

// imports
use chrono::Datelike;

// returns (month, day, year)
pub fn date_parse(tweet: &egg_mode::tweet::Tweet) -> (&str, u32, i32) {
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    (
        months[tweet.created_at.month0() as usize],
        tweet.created_at.day(),
        tweet.created_at.year(),
    )
}

pub fn hello() {
    println!("hello");
}
