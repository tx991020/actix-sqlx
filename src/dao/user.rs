use crate::models::user::{Login, Register,User};
use crate::state::{AppState};
use mobc_redis::{AsyncCommands};


#[async_trait]
pub trait IUser {
    async fn user_add(&self, form: &Register) -> sqlx::Result<u64>;
    async fn user_query(&self, name: &str) -> sqlx::Result<User>;
}


fn passhash(name: &str, pass: &str) -> String {
    let namedpass = format!("{}{}", name, pass);
    let hash = bcrypt::hash(namedpass.as_bytes(), bcrypt::DEFAULT_COST).unwrap();
    // info!("{}{}: {}", name, pass, hash);
    hash
}

fn passhash_verify(name: &str, pass: &str, hash: &str) -> bool {
    let namedpass = format!("{}{}", name, pass);
    bcrypt::verify(namedpass.as_bytes(), hash).unwrap()
}


impl Login {
    pub fn verify(&self, hash: &str) -> bool {
        passhash_verify(&self.name, &self.password, hash)
    }
}


impl Register {
    pub fn passhash(&self) -> String {
        passhash(&self.name, &self.password)
    }
}


#[cfg(any(feature = "mysql"))]
#[async_trait]
impl IUser for AppState {
    async fn user_add(&self, form: &Register) -> sqlx::Result<u64> {
        let passh = form.passhash();


        sqlx::query!(
            r#"
        INSERT INTO users (name, email, pass)
        VALUES (?, ?, ?)
                "#,
            form.name,
            form.email,
            passh
        )
            .execute(&self.sql)
            .await
    }
    async fn user_query(&self, name: &str) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            r#"
        SELECT id, name, email, pass, create_dt, update_dt
        FROM users
        where name = ?
                "#,
            name
        )
            .fetch_one(&self.sql)
            .await
    }
}

#[cfg(any(feature = "postgres"))]
#[async_trait]
impl IUser for AppState {
    async fn user_add(&self, form: &Register) -> sqlx::Result<u64> {
        let passh = form.passhash();

        sqlx::query!(
            r#"
        INSERT INTO users (name, email, pass)
        VALUES ($1 ,$2 ,$3)
                "#,
            form.name,
            form.email,
            passh
        )
            .execute(&self.sql)
            .await
    }
    async fn user_query(&self, name: &str) -> sqlx::Result<User> {
        let mut conn = self.kv.get().await.unwrap();
        let () = conn.set("name", "mobc-redis").await.unwrap();
        let s: String = conn.get("name").await.unwrap();
        info!("{}",s.as_str());



        sqlx::query_as!(
            User,
            r#"
        SELECT id, name, email, pass, create_dt, update_dt
        FROM users
        where name = $1
                "#,
            name
        )
            .fetch_one(&self.sql)
            .await
    }
}

// maybe a bug or should to use string: database couldn't tell us the type of column #5 ("create_dt"); this can happen for columns that are the result of an expression
#[cfg(any(feature = "sqlite"))]
#[async_trait]
impl IUser for AppState {
    async fn user_add(&self, form: &Register) -> sqlx::Result<u64> {
        let passh = form.passhash();

        sqlx::query!(
            r#"
        INSERT INTO users ( name, email, pass )
        VALUES ( $1, $2, $3 )
                "#,
            form.name,
            form.email,
            passh
        )
            .execute(&self.sql)
            .await
            .map(|c| c as _)
    }
    async fn user_query(&self, name: &str) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            r#"
        SELECT id, name, email, pass, create_dt, update_dt
        FROM users
        where name = $1
                "#,
            name
        )
            .fetch_one(&self.sql)
            .await
    }
}


