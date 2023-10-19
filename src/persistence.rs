use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use mysql::params;
use mysql::prelude::Queryable;

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyName,
    MysqlError(mysql::Error),
    Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyName
            => StatusCode::BAD_REQUEST,

            PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

pub fn create_user(pool: &mysql::Pool, name: String) -> Result<(), PersistenceError> {
    if name.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyName);
    }

    let mut conn = pool.get_conn()?;

    let last_insert_id = insert_bank_data(&mut conn, name.to_lowercase())?;

    if last_insert_id > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

fn insert_bank_data(conn: &mut mysql::PooledConn, my_name: String) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "
        INSERT INTO name_details (name)
        VALUES (:name)
        ",
        params! {
            "name"=>my_name
        },
    )
        .map(|_| conn.last_insert_id())
}