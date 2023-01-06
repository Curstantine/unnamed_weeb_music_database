use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RefreshToken {
    pub id: i64,
    pub user_id: String,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub revoked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for RefreshToken {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let user_id: String = row.try_get("user_id")?;
        let token: String = row.try_get("token")?;
        let expires_at: DateTime<Utc> = row.try_get("expires_at")?;
        let revoked: bool = row.try_get("revoked")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let updated_at: DateTime<Utc> = row.try_get("updated_at")?;

        Ok(Self {
            id,
            user_id,
            token,
            expires_at,
            revoked,
            created_at,
            updated_at,
        })
    }
}

#[allow(dead_code)]
pub enum RefreshTokenIden {
    Table,
    Id,
    UserId,
    Token,
    ExpiresAt,
    Revoked,
    CreatedAt,
    UpdatedAt,
}

impl sea_query::Iden for RefreshTokenIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                RefreshTokenIden::Table => "refresh_tokens",
                RefreshTokenIden::Id => "id",
                RefreshTokenIden::UserId => "user_id",
                RefreshTokenIden::Token => "token",
                RefreshTokenIden::ExpiresAt => "expires_at",
                RefreshTokenIden::Revoked => "revoked",
                RefreshTokenIden::CreatedAt => "created_at",
                RefreshTokenIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap();
    }
}
