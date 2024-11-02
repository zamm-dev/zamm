use crate::commands::errors::ZammResult;
use crate::commands::PAGE_SIZE;
use crate::models::asciicasts::AsciiCast;
use crate::models::EntityId;
use crate::schema::asciicasts;
use crate::ZammDatabase;
use anyhow::anyhow;
use asciicast::EventType;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
use specta::specta;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TerminalSessionReference {
    pub id: EntityId,
    pub timestamp: NaiveDateTime,
    pub command: String,
    pub last_io: Option<String>,
}

impl From<AsciiCast> for TerminalSessionReference {
    fn from(value: AsciiCast) -> Self {
        let mut last_io = value
            .cast
            .entries
            .iter()
            .filter(|e| e.event_type == EventType::Input)
            .last()
            .map(|e| e.event_data.clone());
        if last_io.is_none() {
            last_io = value
                .cast
                .entries
                .iter()
                .filter(|e| e.event_type == EventType::Output)
                .last()
                .map(|e| e.event_data.clone());
        }
        TerminalSessionReference {
            id: value.id,
            timestamp: value.timestamp,
            command: value.command,
            last_io,
        }
    }
}

async fn get_terminal_sessions_helper(
    zamm_db: &ZammDatabase,
    offset: i32,
) -> ZammResult<Vec<TerminalSessionReference>> {
    let mut db = zamm_db.0.lock().await;
    let conn = db.as_mut().ok_or(anyhow!("Failed to lock database"))?;
    let result: Vec<AsciiCast> = asciicasts::table
        .order(asciicasts::timestamp.desc())
        .offset(offset as i64)
        .limit(PAGE_SIZE)
        .load::<AsciiCast>(conn)?;
    let calls: Vec<TerminalSessionReference> =
        result.into_iter().map(|row| row.into()).collect();
    Ok(calls)
}

#[tauri::command(async)]
#[specta]
pub async fn get_terminal_sessions(
    database: State<'_, ZammDatabase>,
    offset: i32,
) -> ZammResult<Vec<TerminalSessionReference>> {
    get_terminal_sessions_helper(&database, offset).await
}
