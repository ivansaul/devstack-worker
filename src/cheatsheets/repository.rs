use crate::{
    cheatsheets::models::{CheatsheetMetaRow, CheatsheetRow},
    error::ApiError,
};

pub(crate) struct CheatsheetRepository {
    db: worker::D1Database,
}

impl CheatsheetRepository {
    pub fn new(db: worker::D1Database) -> Self {
        Self { db }
    }

    pub async fn list_meta(&self) -> Result<Vec<CheatsheetMetaRow>, ApiError> {
        let stmt = self
            .db
            .prepare("SELECT id, title, tags, categories, intro, label, icon FROM cheatsheets");
        let query = stmt.bind(&[])?;
        let rows = query.all().await?;
        let records = rows.results::<CheatsheetMetaRow>()?;
        Ok(records)
    }

    pub async fn fetch_by_id(&self, id: &str) -> Result<CheatsheetRow, ApiError> {
        let stmt = self.db.prepare("SELECT * FROM cheatsheets WHERE id = ?");
        let query = stmt.bind(&[id.into()])?;
        let record = query.first::<CheatsheetRow>(None).await?.ok_or_else(|| {
            ApiError::NotFound(format!("The data for key `{id}` is not available"))
        })?;
        Ok(record)
    }
}
