use async_graphql::Object;
use sea_query::Value;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    Decode, FromRow, Row,
};
use ulid::Ulid;

#[derive(
    async_graphql::Enum,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Copy,
    Decode,
    Deserialize,
    Serialize,
    sqlx::Encode,
)]
pub enum AccessLevel {
    Admin,
    Moderator,
    Contributor,
    User,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: Ulid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub access_level: AccessLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(async_graphql::InputObject)]
pub struct Login {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: String,
}

#[derive(async_graphql::InputObject)]
pub struct Register {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(
        username: String,
        email: String,
        password: String,
        access_level: AccessLevel,
    ) -> Self {
        let id = Ulid::new();
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
        let created_at = Utc::now();
        let updated_at = Utc::now();

        Self {
            id,
            username,
            email,
            password_hash,
            access_level,
            created_at,
            updated_at,
        }
    }
}

impl<'r> FromRow<'r, PgRow> for User {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let username: String = row.try_get("username")?;
        let email: String = row.try_get("email")?;
        let password_hash: String = row.try_get("password_hash")?;
        let access_level: AccessLevel = row.try_get("access_level")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let updated_at: DateTime<Utc> = row.try_get("updated_at")?;

        Ok(Self {
            id: id.parse().unwrap(),
            username,
            email,
            password_hash,
            access_level,
            created_at,
            updated_at,
        })
    }
}

//implement Serialize and Deserialize for User
impl serde::Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("User", 7)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("username", &self.username)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("password_hash", &self.password_hash)?;
        state.serialize_field("access_level", &self.access_level)?;
        state.serialize_field("created_at", &self.created_at.to_rfc3339())?;
        state.serialize_field("updated_at", &self.updated_at.to_rfc3339())?;
        state.end()
    }
}

//implement Serialize and Deserialize for User
impl<'de> serde::Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct UserVisitor {
            id: String,
            username: String,
            email: String,
            password_hash: String,
            access_level: AccessLevel,
            created_at: String,
            updated_at: String,
        }

        let visitor = UserVisitor::deserialize(deserializer)?;
        Ok(Self {
            id: visitor.id.parse().unwrap(),
            username: visitor.username,
            email: visitor.email,
            password_hash: visitor.password_hash,
            access_level: visitor.access_level,
            created_at: DateTime::parse_from_rfc3339(&visitor.created_at)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&visitor.updated_at)
                .unwrap()
                .with_timezone(&Utc),
        })
    }
}

#[allow(dead_code)]
pub enum UserIden {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    AccessLevel,
    CreatedAt,
    UpdatedAt,
}

impl sea_query::Iden for UserIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                UserIden::Table => "users",
                UserIden::Id => "id",
                UserIden::Username => "username",
                UserIden::Email => "email",
                UserIden::PasswordHash => "password_hash",
                UserIden::AccessLevel => "access_level",
                UserIden::CreatedAt => "created_at",
                UserIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap();
    }
}

#[Object]
impl User {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn username(&self) -> &str {
        &self.username
    }

    async fn email(&self) -> &str {
        &self.email
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

impl sqlx::Type<sqlx::Postgres> for AccessLevel {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("access_level")
    }
}

impl From<AccessLevel> for Value {
    fn from(access_level: AccessLevel) -> Self {
        match access_level {
            AccessLevel::Admin => "Admin".into(),
            AccessLevel::Moderator => "Moderator".into(),
            AccessLevel::Contributor => "Contributor".into(),
            AccessLevel::User => "User".into(),
        }
    }
}
