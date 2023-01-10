use super::{ExternalSite, Name};
use async_graphql::{Enum, Object};
use sqlx::types::chrono::NaiveDate;
use sqlx::{postgres::PgRow, Decode, FromRow, Row};
use ulid::Ulid;

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, Decode)]
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
    /// Unique ID of the release
    pub id: Ulid,
    /// Name of the release
    pub name: Name,
    /// Type of the release
    pub release_type: ReleaseType,
    /// Total number of tracks in the release
    pub total_tracks: i32,
    /// Date when the release was released
    pub release_date: NaiveDate,
    /// External links to the release
    ///
    /// This is used to link to the release on other platforms such as Spotify,
    /// Apple Music and etc.
    ///
    /// This is optional because not all releases are available on all platforms.
    ///
    /// This is a `Vec` because a release can be available on multiple platforms.
    pub external_sites: Option<Vec<ExternalSite>>,
    /// Label of the release
    pub label: Option<Vec<String>>,
    /// Length of the release in seconds
    pub length: Option<i64>,
    /// Language of the release
    ///
    /// This is a `Vec` because a release can include multiple languages.
    pub script_language: Option<Vec<String>>,
}

impl<'r> FromRow<'r, PgRow> for Release {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let name: Name = row.try_get("name")?;
        let release_type: ReleaseType = row.try_get("release_type")?;
        let total_tracks: i32 = row.try_get("total_tracks")?;
        let release_date: NaiveDate = row.try_get("release_date")?;
        let external_sites: Option<Vec<ExternalSite>> = row.try_get("external_sites")?;
        let label: Option<Vec<String>> = row.try_get("label")?;
        let length: Option<i64> = row.try_get("total_length")?;
        let script_language: Option<Vec<String>> = row.try_get("script_language")?;

        Ok(Self {
            id: Ulid::from_string(&id).unwrap(),
            name,
            release_type,
            total_tracks,
            release_date,
            external_sites,
            label,
            length,
            script_language,
        })
    }
}

// Ignore unused enum variants
#[allow(dead_code)]
pub enum ReleaseIden {
    Table,
    Id,
    Name,
    ReleaseType,
    TotalTracks,
    ReleaseDate,
    ExternalSites,
    Label,
    Length,
    ScriptLanguage,
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
                ReleaseIden::ReleaseDate => "release_date",
                ReleaseIden::ExternalSites => "external_sites",
                ReleaseIden::Label => "label",
                ReleaseIden::Length => "length",
                ReleaseIden::ScriptLanguage => "script_language",
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
                SongReleaseIden::SongId => "song_id",
                SongReleaseIden::ReleaseId => "release_id",
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

    async fn release_date(&self) -> &NaiveDate {
        &self.release_date
    }

    async fn external_sites(&self) -> Option<&Vec<ExternalSite>> {
        self.external_sites.as_ref()
    }

    async fn label(&self) -> Option<&Vec<String>> {
        self.label.as_ref()
    }

    async fn length(&self) -> Option<&i64> {
        self.length.as_ref()
    }

    async fn script_language(&self) -> Option<&Vec<String>> {
        self.script_language.as_ref()
    }
}

/// Options for [`Release::get_releases`]

#[derive(Clone, Debug)]
pub struct Options {
    pub id: Option<String>,
    pub search: Option<String>,
    pub artist_id: Option<String>,
    pub song_id: Option<String>,
    pub genres: Option<Vec<String>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

// Implement Type for ReleaseType
//
// This is requires for ReleaseType to be decoded properly with sqlx
impl sqlx::Type<sqlx::Postgres> for ReleaseType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("release_type")
    }
}
