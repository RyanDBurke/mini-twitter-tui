/* ================= MAIN =================*/

// personal imports
mod ui;
mod util;
mod config;
mod test;

#[tokio::main]
async fn main() {

    let config = config::config::Config::load().await; //util::misc::get_config();
    let users = util::user::get_user(&config, &(config.screen_name));
    let t = util::user::user_to_string(&config, users).await;
    
    println!("{}", t);
    ui::ui::build_ui().expect("User Interace failed to build.");
}
