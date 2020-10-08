/* ================= MAIN =================*/

// personal imports
mod ui;
mod util;
mod config;
mod test;

#[tokio::main]
async fn main() {

    let config = config::config::Config::load().await; //util::misc::get_config();
    let t = util::tweet::print_home_timeline(&config, 5).await;

    match t {
        Ok(v) => {
            for tweet in v {
                println!("{}\n", tweet);
            }
        },
        Err(_) => println!("Error"),
    }


    ui::ui::build_ui().expect("User Interace failed to build.");
}
