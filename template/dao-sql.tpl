use crate::state::{AppState};
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use futures::future::{ready, Ready};
use anyhow::Result;



// this struct will use to receive user input
#[derive(Serialize, Deserialize)]
pub struct TableRequest {
    pub description: String,
    pub done: bool,
}

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct Table {
    pub id: i32,
    pub description: String,
    pub done: bool,
}


impl {{Table}} {
    pub async fn query(query: &str, state: AppState) -> Result<Vec<Self>> {
        let recs = sqlx::query_as!({{Table}},r#"
                    SELECT * FROM {{Table}}
                "#)
            .fetch_all(&state.sql)
            .await?;

        Ok(recs)
    }

    pub async fn find_by_id(id: i32, state: AppState) -> Result<Self> {
        let rec = sqlx::query!(
                r#"
                    SELECT * FROM {{table}} WHERE id = $1
                "#,
                id
            )
            .fetch_one(&state.sql)
            .await?;
        Ok({{Table}} {
            id: rec.id,
            description: rec.description,
            done: rec.done,
        })
    }

    pub async fn create({{table}}: {{Table}}Request, state: AppState) -> Result<Self> {
        let mut tx = state.sql.begin().await?;
        let {{table}} = sqlx::query("INSERT INTO {{table}} (description, done) VALUES ($1, $2) RETURNING id, description, done")
            .bind(&{{table}}.description)
            .bind({{table}}.done)
            .map(|row: PgRow| {
                {{Table}} {
                    id: row.get(0),
                    description: row.get(1),
                    done: row.get(2),
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok({{table}})
    }

    pub async fn update(id: i32, {{table}}: {{Table}}, state: AppState) -> Result<Self> {
        let mut tx = state.sql.begin().await.unwrap();
        let {{table}} = sqlx::query("UPDATE {{table}} SET description = $1, done = $2 WHERE id = $3 RETURNING id, description, done")
            .bind(&{{table}}.description)
            .bind({{table}}.done)
            .bind(id)
            .map(|row: PgRow| {
                {{Table}} {
                    id: row.get(0),
                    description: row.get(1),
                    done: row.get(2),
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await.unwrap();
        Ok({{table}})
    }

    pub async fn delete(id: i32, state: AppState) -> Result<u64> {
        let mut tx = state.sql.begin().await?;
        let deleted = sqlx::query("DELETE FROM {{table}} WHERE id = $1")
            .bind(id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(deleted)
    }
}
