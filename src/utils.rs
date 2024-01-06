use crate::channel::Channel;
use crate::history::{History, VideoType};
use std::collections::{HashMap, HashSet};

pub fn get_total_videos(histories: &[History], format: VideoType) -> u32 {
    histories.iter().filter(|h| h.video_type == format).count() as u32
}

pub fn get_top_five(histories: &[History], format: VideoType) -> Vec<(Channel, u32)> {
    let m =
        histories
            .iter()
            .filter(|h| h.video_type == format)
            .fold(HashMap::new(), |mut acc, h| {
                let ch = Channel {
                    title: h.subtitles[0].name.clone(),
                    url: h.subtitles[0].url.clone(),
                };
                *acc.entry(ch).or_insert(0_u32) += 1_u32;
                acc
            });
    let mut count_vec: Vec<_> = m.into_iter().collect();
    dbg!(&count_vec.len());
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    count_vec
        .into_iter()
        .take(5)
        .collect::<Vec<(Channel, u32)>>()
}

pub fn get_total_creators(histories: &[History], format: VideoType) -> u32 {
    let s =
        histories
            .iter()
            .filter(|h| h.video_type == format)
            .fold(HashSet::new(), |mut acc, h| {
                let _ = acc.insert(&h.subtitles[0].url);
                acc
            });
    s.len() as u32
}
