/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use uuid::Uuid;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::{
    db::DBConnection,
    errors::Error,
    models::user::User,
    schema::{
        protein_logs,
        protein_logs::dsl::{log_id, user_id},
    },
};

// Nutrition Protein Logs
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    Identifiable,
    Insertable,
    Associations,
)]
#[diesel(primary_key(log_id))]
#[diesel(table_name = protein_logs)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProteinLog {
    log_id: i32,
    user_id: Uuid,
    date: NaiveDateTime,
    amount: i32,
    updated_at: NaiveDateTime,
}

impl ProteinLog {
    pub async fn find(
        user: Uuid,
        id: i32,
        connection: &mut DBConnection,
    ) -> Result<ProteinLog, Error> {
        protein_logs::table
            .filter(user_id.eq(user))
            .filter(log_id.eq(id))
            .select(ProteinLog::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DBConnection) -> Result<Vec<ProteinLog>, Error> {
        protein_logs::table
            .select(ProteinLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn user_all(
        user: Uuid,
        connection: &mut DBConnection,
    ) -> Result<Vec<ProteinLog>, Error> {
        protein_logs::table
            .filter(user_id.eq(user))
            .select(ProteinLog::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        user: Uuid,
        id: i32,
        data: UpdateProteinLog,
        connection: &mut DBConnection,
    ) -> Result<ProteinLog, Error> {
        diesel::update(protein_logs::table)
            .filter(user_id.eq(user))
            .filter(log_id.eq(id))
            .set(&data)
            .get_result::<ProteinLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(
        user: Uuid,
        id: i32,
        connection: &mut DBConnection,
    ) -> Result<usize, Error> {
        diesel::delete(protein_logs::table)
            .filter(user_id.eq(user))
            .filter(log_id.eq(id))
            .execute(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(
        data: NewProteinLog,
        connection: &mut DBConnection,
    ) -> Result<ProteinLog, Error> {
        diesel::insert_into(protein_logs::table)
            .values(&data)
            .get_result::<ProteinLog>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = protein_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProteinLog {
    pub amount: Option<i32>,
    pub date: Option<NaiveDateTime>,
}

#[derive(AsChangeset)]
#[diesel(table_name = protein_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct NewProteinLog {
    pub user_id: Uuid,
    pub amount: i32,
}
