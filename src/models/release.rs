use super::Name;
use async_graphql::{Enum, Object};
use sqlx::{postgres::PgRow, FromRow, Row};
use ulid::Ulid;

#[derive(Enum, Copy, Clone, Debug, sqlx::Type, Eq, PartialEq)]
pub enum ReleaseType {
    Album,
    Single,
    EP,
}

#[derive(Clone, Debug)]
/// Release done by one or multiple artist
///
/// This structure simply represents an album but has a fancy name to not to
/// confuse it with [`ReleaseType::Album`]
pub struct Release {
    pub id: Ulid,
    pub name: Name,
    pub release_type: ReleaseType,
    pub total_tracks: i32,
}

impl<'r> FromRow<'r, PgRow> for Release {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get(0)?;
        let name: Name = row.try_get(1)?;
        let release_type: ReleaseType = row.try_get(2)?;
        let total_tracks: i32 = row.try_get(3)?;

        Ok(Self {
            id: Ulid::from_string(&id).unwrap(),
            name,
            release_type,
            total_tracks,
        })
    }
}

pub enum ReleaseIden {
    Table,
    Id,
    Name,
    ReleaseType,
    TotalTracks,
}

impl sea_query::Iden for ReleaseIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                ReleaseIden::Table => "releases",
                ReleaseIden::Id => "id",
                ReleaseIden::Name => "name",
                ReleaseIden::ReleaseType => "release_type",
                ReleaseIden::TotalTracks => "total_tracks",
            }
        )
        .unwrap();
    }
}

pub enum SongReleaseIden {
    Table,
    SongId,
    ReleaseId,
}

impl sea_query::Iden for SongReleaseIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                SongReleaseIden::Table => "songs_releases",
                SongReleaseIden::SongId => "songs_id",
                SongReleaseIden::ReleaseId => "releases_id",
            }
        )
        .unwrap();
    }
}

#[Object]
impl Release {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn name(&self) -> &Name {
        &self.name
    }

    async fn release_type(&self) -> &ReleaseType {
        &self.release_type
    }

    async fn total_tracks(&self) -> &i32 {
        &self.total_tracks
    }

    /*fn artists(&self, context: &Context) -> Vec<Artist> {
        Artist::get_artists_by_release_id(&self.id, context.db).unwrap()
    }

    fn songs(&self, context: &Context) -> Vec<Song> {
        Song::get_songs_by_release_id(&self.id, context.db).unwrap()
    }*/
}
