use anyhow::Result;
use cheatsheets::models::{Cheatsheet, CheatsheetSeed};
use futures::{StreamExt, stream};
use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let db_path = "cheatsheets.db";
    let pool = create_pool(db_path).await?;

    create_table(&pool).await?;

    let ids = enabled_cheatsheet_ids()?;
    let mut results = Vec::new();
    let mut stream = stream::iter(ids.iter())
        .map(|id| cheatsheets::parse_markdown(id))
        .buffer_unordered(10);

    while let Some(res) = stream.next().await {
        results.push(res?);
    }

    for cheatsheet in results.iter() {
        insert_row(&pool, cheatsheet).await?;
    }

    export_sql(db_path, "schema.sql")?;
    prepend_sql("schema.sql")?;

    println!("schema.sql generated successfully 🔥");

    Ok(())
}

async fn create_pool(db_path: &str) -> Result<Pool<Sqlite>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(
            SqliteConnectOptions::new()
                .filename(db_path)
                .create_if_missing(true),
        )
        .await?;
    Ok(pool)
}

async fn create_table(pool: &Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
        DROP TABLE IF EXISTS cheatsheets;
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
    CREATE TABLE cheatsheets (
        id TEXT NOT NULL PRIMARY KEY,
        title TEXT NOT NULL,
        tags JSON,
        categories JSON,
        intro TEXT,
        label TEXT,
        icon TEXT,
        sections JSON NOT NULL
    );
    "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_row(pool: &Pool<Sqlite>, cheatsheet: &Cheatsheet) -> Result<()> {
    sqlx::query(
        r#"
    INSERT INTO cheatsheets
    (id, title, tags, categories, intro, label, icon, sections)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?);
    "#,
    )
    .bind(&cheatsheet.id)
    .bind(&cheatsheet.title)
    .bind(serde_json::to_string(&cheatsheet.tags)?)
    .bind(serde_json::to_string(&cheatsheet.categories)?)
    .bind(&cheatsheet.intro)
    .bind(&cheatsheet.label)
    .bind(&cheatsheet.icon)
    .bind(serde_json::to_string(&cheatsheet.sections)?)
    .execute(pool)
    .await?;
    Ok(())
}

fn export_sql(db_path: &str, out: &str) -> Result<()> {
    let output = Command::new("sqlite3").arg(db_path).arg(".dump").output()?;
    std::fs::write(out, output.stdout)?;
    Ok(())
}

fn prepend_sql(path: &str) -> Result<()> {
    let mut content = std::fs::read_to_string(path)?;
    content.insert_str(0, "DROP TABLE IF EXISTS cheatsheets;\n");
    std::fs::write(path, content)?;
    Ok(())
}

fn enabled_cheatsheet_ids() -> Result<Vec<String>> {
    let seeds = serde_yml::from_str::<Vec<CheatsheetSeed>>(include_str!("seed.yml"))?;
    let ids = seeds
        .into_iter()
        .filter(|seed| seed.enabled)
        .map(|seed| seed.id)
        .collect();
    Ok(ids)
}
