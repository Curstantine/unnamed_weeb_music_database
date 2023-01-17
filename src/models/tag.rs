use async_graphql::{InputObject, Object};

use sea_query::Iden;
use serde::Deserialize;
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};
use ulid::Ulid;

#[derive(Clone, Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, InputObject)]
pub struct NewTag {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, InputObject)]
pub struct UpdateTag {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub song_id: Option<Ulid>,
    pub release_id: Option<Ulid>,
}

#[allow(dead_code)]
pub enum TagIden {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

pub enum ReleaseTagIden {
    Table,
    ReleaseId,
    TagId,
}

pub enum SongTagIden {
    Table,
    SongId,
    TagId,
}

impl<'r> FromRow<'r, PgRow> for Tag {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[Object]
impl Tag {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Iden for TagIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                TagIden::Table => "tags",
                TagIden::Id => "id",
                TagIden::Name => "name",
                TagIden::Description => "description",
                TagIden::CreatedAt => "created_at",
                TagIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap();
    }
}

impl Iden for ReleaseTagIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                ReleaseTagIden::Table => "release_tags",
                ReleaseTagIden::ReleaseId => "release_id",
                ReleaseTagIden::TagId => "tag_id",
            }
        )
        .unwrap();
    }
}

impl Iden for SongTagIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                SongTagIden::Table => "song_tags",
                SongTagIden::SongId => "song_id",
                SongTagIden::TagId => "tag_id",
            }
        )
        .unwrap();
    }
}

impl Options {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            song_id: None,
            release_id: None,
        }
    }

    pub fn id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn song_id(mut self, song_id: Ulid) -> Self {
        self.song_id = Some(song_id);
        self
    }

    pub fn release_id(mut self, release_id: Ulid) -> Self {
        self.release_id = Some(release_id);
        self
    }
}
