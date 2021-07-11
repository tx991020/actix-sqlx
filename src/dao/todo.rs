use crate::state::{AppState};
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use futures::future::{ready, Ready};
use anyhow::Result;


// this struct will be used to represent database record
#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}


#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    pub description: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct TodoFilter {
    pub from: Option<i32>,
    pub limit: Option<i32>,
    pub filter: Option<String>,
}


impl Todo {
    pub async fn query(query: TodoFilter, state: AppState) -> Result<Vec<Self>> {
        let recs = sqlx::query_as!(Todo," SELECT * FROM todo order by id desc limit 10")
            .fetch_all(&state.sql)
            .await?;

        Ok(recs)
    }

    pub async fn find_by_id(id: i32, state: AppState) -> Result<Self> {
        let res = sqlx::query_as!(Todo,
                r#"
                    SELECT * FROM todo WHERE id = $1
                "#,
                id
            )
            .fetch_one(&state.sql)
            .await?;
        Ok(res)
    }

    pub async fn create(todo: TodoRequest, state: AppState) -> Result<Self> {
        let mut tx = state.sql.begin().await?;
        let todo = sqlx::query("INSERT INTO todo (description, done) VALUES ($1, $2) RETURNING id, description, done")
            .bind(&todo.description)
            .bind(todo.done)
            .map(|row: PgRow| {
                Todo {
                    id: row.get(0),
                    description: row.get(1),
                    done: row.get(2),
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(todo)
    }

    pub async fn update(id: i32, todo: TodoRequest, state: AppState) -> Result<Self> {
        let mut tx = state.sql.begin().await.unwrap();
        let todo = sqlx::query("UPDATE todo SET description = $1, done = $2 WHERE id = $3 RETURNING id, description, done")
            .bind(&todo.description)
            .bind(todo.done)
            .bind(id)
            .map(|row: PgRow| {
                Todo {
                    id: row.get(0),
                    description: row.get(1),
                    done: row.get(2),
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await.unwrap();
        Ok(todo)
    }

    pub async fn delete(id: i32, state: AppState) -> Result<u64> {
        let mut tx = state.sql.begin().await?;
        let deleted = sqlx::query("DELETE FROM todo WHERE id = $1")
            .bind(id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(deleted)
    }
}



