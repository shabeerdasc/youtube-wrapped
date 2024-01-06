use dotenv;
use youtube_wrapped::history::{History, VideoType};
use youtube_wrapped::utils::get_top_five;
// use youtube_wrapped::utils::get_total_creators;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let watch_history = include_str!("../data/final.json");
    let histories: Vec<History> = serde_json::from_str(&watch_history).unwrap();

    let top_channels = get_top_five(&histories, VideoType::Normal);

    let custom = reqwest::redirect::Policy::none();
    let client = reqwest::Client::builder().redirect(custom).build().unwrap();

    let mut top_five = vec![];
    for (c, count) in top_channels {
        let info = c.channel_info(&client).await;
        top_five.push((info, count));
    }

    println!("{:?}", top_five);
    // let total = get_total_creators(&histories, VideoType::Normal);
    // println!("{}", total);
}
