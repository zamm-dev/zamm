use specta::specta;

use tauri::State;

use crate::commands::errors::ZammResult;
use crate::models::database_contents::read_database_contents;
use crate::ZammDatabase;

async fn import_db_helper(zamm_db: &ZammDatabase, db_path: String) -> ZammResult<()> {
    read_database_contents(zamm_db, &db_path).await
}

#[tauri::command(async)]
#[specta]
pub async fn import_db(
    database: State<'_, ZammDatabase>,
    db_path: String,
) -> ZammResult<()> {
    import_db_helper(&database, db_path).await
}
