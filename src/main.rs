use anyhow::{Context, Result};
use data::GameData;
use roga::*;
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter};
use std::fs;
use transport::console::ConsoleTransport;
use uuid::Uuid;

mod data;
mod entity;

#[tokio::main]
async fn main() -> Result<()> {
    let logger = Logger::new()
        .with_transport(ConsoleTransport {
            ..Default::default()
        })
        .with_level(LoggerLevel::Info)
        .with_label("GalKeeper Migration");

    l_info!(logger, "Starting WhiteCloud to GalKeeper migration...");

    let conn = Database::connect("sqlite://./db.3.sqlite")
        .await
        .context("Failed to connect to WhiteCloud database")?;

    l_info!(logger, "Connected to WhiteCloud database");

    let games: Vec<entity::games::Model> = entity::games::Entity::find()
        .all(&conn)
        .await
        .context("Failed to fetch games")?;

    l_info!(logger, "Found {} games in WhiteCloud database", games.len());

    let mut migrated_games = Vec::new();

    for game in games {
        let uuid = game.uuid.unwrap_or_default();
        let name = game.name.unwrap_or_default();
        if uuid.is_empty() {
            continue;
        }

        let play_records: Vec<entity::history::Model> = entity::history::Entity::find()
            .filter(entity::history::Column::Game.eq(uuid.clone()))
            .all(&conn)
            .await
            .context("Failed to fetch play records")?;
        l_info!(
            logger,
            "Found {} play records for game {} ({})",
            play_records.len(),
            name,
            uuid
        );

        let mut last_play: i64 = 0;

        let timelines: Vec<(i64, i64, i64)> = play_records
            .iter()
            .filter_map(|record| {
                if let (Some(start), Some(end)) = (record.start, record.end) {
                    let start_ts = start.try_into().unwrap_or(0);
                    let end_ts = end.try_into().unwrap_or(0);
                    let duration = end_ts - start_ts;
                    last_play = end_ts.max(last_play);

                    if start_ts > 0 && end_ts > 0 && duration > 0 {
                        Some((start_ts / 1000, end_ts / 1000, duration / 1000));
                    }
                }
                None
            })
            .collect();

        migrated_games.push(GameData {
            id: Uuid::new_v4().to_string(),
            vndb_id: None,
            bgm_id: None,
            update_date: chrono::Utc::now().timestamp_millis(),
            title: name,
            alias: Vec::new(),
            cover: String::new(),
            description: String::new(),
            tags: Vec::new(),
            play_timelines: timelines,
            expected_play_hours: 0.0,
            last_play,
            create_date: game
                .update_time
                .and_then(|t| t.try_into().ok())
                .unwrap_or_else(|| chrono::Utc::now().timestamp_millis()),
            release_date: 0,
            rating: 0.0,
            developer: String::new(),
            images: Vec::new(),
            links: Vec::new(),
        });
    }

    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
    let output_file = format!("export-from-whitecloud-{}.json", timestamp);
    fs::write(&output_file, serde_json::to_string_pretty(&migrated_games)?)?;
    l_info!(
        logger,
        "Successfully migrated {} games to {}",
        migrated_games.len(),
        output_file
    );

    Ok(())
}
