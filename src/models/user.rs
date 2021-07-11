


#[cfg(any(feature = "mysql"))]
type SqlID = u64;
#[cfg(any(feature = "sqlite", feature = "postgres"))]
type SqlID = i32;

// time_zone: https://github.com/launchbadge/sqlx/issues/329
#[cfg(any(feature = "mysql", feature = "postgres"))]
type SqlDateTime = chrono::DateTime<chrono::Utc>;
// type SqlDateTime = chrono::NaiveDateTime;

// Extend derive(FromRow): https://github.com/launchbadge/sqlx/issues/156
#[cfg(any(feature = "sqlite"))]
type SqlDateTime = String;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: SqlID,
    pub name: String,
    // pub phone: String,
    pub email: String,
    // not return password
    #[serde(skip_serializing)]
    pub pass: String,
    pub create_dt: SqlDateTime,
    pub update_dt: SqlDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub name: String,
    pub password: String,
    #[serde(default)]
    pub rememberme: bool,
}



#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Claims {
    // username
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}



