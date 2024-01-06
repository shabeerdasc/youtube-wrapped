use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use url::Url;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct History {
    pub header: String,
    pub title: String,
    pub title_url: String,
    pub subtitles: Vec<ChannelInfo>,
    pub video_type: VideoType,
    pub time: String,
    pub products: Vec<String>,
    pub activity_controls: Vec<String>,
    #[serde(skip_serializing)]
    pub details: Vec<Detail>,
}

impl History {
    pub fn get_video_id(&self) -> Option<String> {
        if !self.title_url.is_empty() {
            let url = Url::try_from(self.title_url.as_str()).unwrap();
            if let Some(query) = url.query() {
                let my_query = serde_qs::from_str::<Query>(query);
                match my_query {
                    Ok(q) => return Some(q.id),
                    Err(_) => return None,
                };
            }
        }
        None
    }
}

#[derive(Deserialize)]
struct Query {
    #[serde(rename = "v")]
    id: String,
}

#[derive(Deserialize, Debug)]
pub struct Detail {
    _name: String,
}

#[derive(Debug, Default, Serialize, PartialEq, Deserialize)]
pub enum VideoType {
    Short,
    Normal,
    #[default]
    Other,
}

#[derive(Deserialize, Debug, Default, Serialize)]
#[serde(default)]
pub struct ChannelInfo {
    pub name: String,
    pub url: String,
}

pub async fn get_type(video_id: Option<String>, client: &reqwest::Client) -> VideoType {
    if let Some(vid_id) = video_id {
        let base_url = "https://www.youtube.com/shorts/";
        // let res = client.get(format!("{base_url}{vid_id}").as_str()).send();
        let res = client
            .get(format!("{base_url}{vid_id}").as_str())
            .send()
            .await;
        match res {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => return VideoType::Short,
                reqwest::StatusCode::SEE_OTHER => return VideoType::Normal,
                _ => VideoType::Other,
            },
            Err(_) => VideoType::Other,
        }
    } else {
        return VideoType::Other;
    }
}

// get video type(Shorts or Normal) and save it in a file. Running in chunks manually
pub async fn get_video_type(infile: &str, outfile: &str) {
    let watch_history = std::fs::read_to_string(infile).unwrap();
    // let watch_history = include_str!(infile);
    let mut histories: Vec<History> = serde_json::from_str(&watch_history).unwrap();
    let custom = reqwest::redirect::Policy::none();
    let client = reqwest::Client::builder().redirect(custom).build().unwrap();

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(outfile)
        .unwrap();

    for h in histories.iter_mut().skip(30000).take(2000) {
        let typ = get_type(h.get_video_id(), &client).await;
        h.video_type = typ;
    }

    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(
        &mut writer,
        &histories
            .iter()
            .skip(30000)
            .take(2000)
            .collect::<Vec<&History>>(),
    )
    .unwrap();
    writer.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_id_works() {
        let watch_history: &str = include_str!("../data/watch-history.json");
        let histories: Vec<History> = serde_json::from_str(&watch_history).unwrap();
        let history = &histories[0];
        assert_eq!(history.get_video_id(), Some("7DKv5H5Frt0".into()));
    }
}
