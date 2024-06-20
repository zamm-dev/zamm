use specta::specta;

use tauri::State;

use crate::commands::errors::ZammResult;
use crate::models::database_contents::write_database_contents;
use crate::ZammDatabase;

async fn export_db_helper(zamm_db: &ZammDatabase, db_path: String) -> ZammResult<()> {
    write_database_contents(zamm_db, &db_path).await
}

#[tauri::command(async)]
#[specta]
pub async fn export_db(
    database: State<'_, ZammDatabase>,
    db_path: String,
) -> ZammResult<()> {
    export_db_helper(&database, db_path).await
}
