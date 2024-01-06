use serde::Deserialize;
use std::env;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Channel {
    pub title: String,
    pub url: String,
}

impl Channel {
    fn get_channel_id(&self) -> String {
        self.url
            .strip_prefix("https://www.youtube.com/channel/")
            .unwrap()
            .to_string()
    }

    pub async fn channel_info(&self, client: &reqwest::Client) -> ChannelDetail {
        let id = self.get_channel_id();
        let channel = get_channel_info(&id, client).await.unwrap();
        channel.get_channel_details()
    }
}

#[derive(Debug)]
pub struct ChannelDetail {
    pub id: String,
    pub name: String,
    pub image: String,
    pub country: String,
    pub topics: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ChannelInfos {
    items: Vec<Item>,
}

impl ChannelInfos {
    fn get_channel_details(&self) -> ChannelDetail {
        ChannelDetail {
            id: self.items[0].id.clone(),
            name: self.items[0].snippet.title.clone(),
            image: self.items[0].snippet.thumbnails.medium.url.clone(),
            country: self.items[0].snippet.country.clone(),
            topics: self.items[0]
                .topic_details
                .topic_categories
                .iter()
                .map(|s| {
                    s.strip_prefix("https://en.wikipedia.org/wiki/")
                        .unwrap()
                        .to_string()
                })
                .collect::<Vec<String>>(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Item {
    id: String,
    snippet: Snippet,
    topic_details: TopicDetails,
}

#[derive(Deserialize, Debug)]
struct Snippet {
    title: String,
    thumbnails: Thumbnail,
    #[serde(default)]
    country: String,
}

#[derive(Deserialize, Debug)]
struct Thumbnail {
    medium: Image,
}

#[derive(Deserialize, Debug)]
struct Image {
    url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TopicDetails {
    topic_categories: Vec<String>,
}

async fn get_channel_info(
    channel_id: &str,
    client: &reqwest::Client,
) -> Result<ChannelInfos, &'static str> {
    let url = format!(
        "https://youtube.googleapis.com/youtube/v3/channels?part=snippet%2CtopicDetails&id={channel_id}&key={}",
        env::var("APIKEY").unwrap()
    );
    let res = client.get(url.as_str()).send().await;
    match res {
        Ok(response) => Ok(response.json::<ChannelInfos>().await.unwrap()),
        Err(_) => Err("Error while parsing"),
    }
}
