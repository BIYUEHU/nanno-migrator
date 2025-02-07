use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameLink {
    pub url: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GameData {
    pub id: String,
    #[serde(rename = "vndbId")]
    pub vndb_id: Option<String>,
    #[serde(rename = "bgmId")]
    pub bgm_id: Option<String>,
    #[serde(rename = "updateDate")]
    pub update_date: i64,
    pub title: String,
    pub alias: Vec<String>,
    pub cover: String,
    pub description: String,
    pub tags: Vec<String>,
    #[serde(rename = "playTimelines")]
    pub play_timelines: Vec<(i64, i64, i64)>,
    #[serde(rename = "expectedPlayHours")]
    pub expected_play_hours: f64,
    #[serde(rename = "lastPlay")]
    pub last_play: i64,
    #[serde(rename = "createDate")]
    pub create_date: i64,
    #[serde(rename = "releaseDate")]
    pub release_date: i64,
    pub rating: f64,
    pub developer: String,
    pub images: Vec<String>,
    pub links: Vec<GameLink>,
}
