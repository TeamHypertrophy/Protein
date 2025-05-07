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
use diesel_derive_enum::DbEnum;
use diesel_async::RunQueryDsl;
use validator::Validate;
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::{db::DBConnection, errors::Error, schema::users, utils};

// User Model
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    AsChangeset,
    Identifiable,
)]
#[diesel(primary_key(user_id))]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub email_verified: bool,
    pub email_verified_at: Option<NaiveDateTime>,
    pub email_verification_token: Uuid,
    pub mfa_enabled: bool,
    pub mfa_code: Option<String>,
    pub mfa_verified: bool,
    pub mfa_verification_token: Uuid,
    pub mfa_code_expires_at: Option<NaiveDateTime>,
    pub password_updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_login: NaiveDateTime,
    pub last_login_ip: String,
    pub ip_address: String,
    pub role: Role,
    pub status: UserStatus,
}

#[derive(Serialize, Deserialize)]
pub struct ProteinUser {
    pub user: User,
    pub api_key: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateUser {
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub email_verified_at: Option<NaiveDateTime>,
    pub email_verification_token: Option<Uuid>,
    pub password_updated_at: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
    pub last_login_ip: Option<String>,
    pub status: Option<UserStatus>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub password: String,
    #[validate(email)]
    pub email: String,
    pub last_login_ip: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Password {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ForgotPassword {
    pub password: String,
    pub email: String,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Role"]
pub enum Role {
    Developer,
    Admin,
    Trainer,
    User,
}

#[derive(DbEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Userstatus"]
pub enum UserStatus {
    Active,
    Pending,
}

impl User {
    pub async fn find(id: Uuid, connection: &mut DBConnection) -> Result<User, Error> {
        users::table
            .find(id)
            .select(User::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn find_by_username(
        name: String,
        connection: &mut DBConnection,
    ) -> Result<User, Error> {
        users::table
            .filter(users::username.eq(name))
            .select(User::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn find_by_email(
        address: String,
        connection: &mut DBConnection,
    ) -> Result<User, Error> {
        users::table
            .filter(users::email.eq(address))
            .select(User::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn find_by_email_verification_token(
        token: Uuid,
        connection: &mut DBConnection,
    ) -> Result<User, Error> {
        users::table
            .filter(users::email_verification_token.eq(token))
            .select(User::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn find_by_mfa_verification_token(
        token: Uuid,
        connection: &mut DBConnection,
    ) -> Result<User, Error> {
        users::table
            .filter(users::mfa_verification_token.eq(token))
            .select(User::as_select())
            .first(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn all(connection: &mut DBConnection) -> Result<Vec<User>, Error> {
        users::table
            .select(User::as_select())
            .load(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn delete(id: Uuid, connection: &mut DBConnection) -> Result<User, Error> {
        diesel::delete(users::table.filter(users::user_id.eq(id)))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update(
        id: Uuid,
        data: UpdateUser,
        connection: &mut DBConnection,
    ) -> Result<User, Error> {
        diesel::update(users::table)
            .filter(users::user_id.eq(id))
            .set(&data)
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn create(connection: &mut DBConnection, data: NewUser) -> Result<User, Error> {
        diesel::insert_into(users::table)
            .values(&data)
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update_password(
        id: Uuid,
        new_password: &String,
        connection: &mut DBConnection,
    ) -> Result<User, Error> {
        diesel::update(users::table)
            .filter(users::user_id.eq(id))
            .set((
                users::password.eq(new_password),
                users::password_updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn update_ip_info(
        id: Uuid,
        new_ip: &String,
        new_last_login: NaiveDateTime,
        connection: &mut DBConnection,
    ) -> Result<User, Error> {
        diesel::update(users::table)
            .filter(users::user_id.eq(id))
            .set((
                users::last_login.eq(new_last_login),
                users::last_login_ip.eq(new_ip),
                users::ip_address.eq(new_ip),
            ))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn verify_email(id: Uuid, connection: &mut DBConnection) -> Result<User, Error> {
        diesel::update(users::table)
            .filter(users::user_id.eq(id))
            .set((
                users::email_verified.eq(true),
                users::status.eq(UserStatus::Active),
                users::email_verified_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn generate_mfa_code(id: Uuid, connection: &mut DBConnection) -> Result<User, Error> {
        let code = utils::password::random();
        let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::minutes(10);

        diesel::update(users::table)
            .filter(users::user_id.eq(id))
            .set((
                users::mfa_code.eq(code),
                users::mfa_code_expires_at.eq(expires_at),
            ))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn enable_mfa(id: Uuid, connection: &mut DBConnection) -> Result<User, Error> {
        diesel::update(users::table)
            .filter(users::user_id.eq(id))
            .set((
                users::mfa_enabled.eq(true),
                users::mfa_code.eq(None::<String>),
                users::mfa_verified.eq(false),
                users::mfa_code_expires_at.eq(None::<NaiveDateTime>),
            ))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn verify_mfa(id: Uuid, connection: &mut DBConnection) -> Result<User, Error> {
        diesel::update(users::table)
            .filter(users::user_id.eq(id))
            .set((
                users::mfa_verified.eq(true),
                users::mfa_code.eq(None::<String>),
                users::mfa_code_expires_at.eq(None::<NaiveDateTime>),
            ))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn reset_mfa(id: Uuid, connection: &mut DBConnection) -> Result<User, Error> {
        diesel::update(users::table)
            .filter(users::user_id.eq(id))
            .set((
                users::mfa_code.eq(None::<String>),
                users::mfa_code_expires_at.eq(None::<NaiveDateTime>),
            ))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn disable_mfa(id: Uuid, connection: &mut DBConnection) -> Result<User, Error> {
        diesel::update(users::table)
            .filter(users::user_id.eq(id))
            .set((
                users::mfa_enabled.eq(false),
                users::mfa_code.eq(None::<String>),
                users::mfa_verified.eq(false),
                users::mfa_code_expires_at.eq(None::<NaiveDateTime>),
            ))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }

    pub async fn elevate(id: Uuid, connection: &mut DBConnection) -> Result<User, Error> {
        diesel::update(users::table)
            .filter(users::user_id.eq(id))
            .set(users::role.eq(Role::Admin))
            .get_result::<User>(connection)
            .await
            .map_err(|error| {
                tracing::error!("[!] PostgreSQL Error: {:?}", error);
                Error::Database(error.to_string())
            })
    }
}
