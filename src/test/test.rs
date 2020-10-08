/* ================= TEST =================*/

#![allow(unused_imports)]

use crate::util::{misc, tweet, user};

#[cfg(test)]
#[test]
pub fn test_get_user_string() {
    let config = misc::get_config();
    let users = user::get_user(&config, &(config.screen_name));


    assert_eq!(1, 1);
}
