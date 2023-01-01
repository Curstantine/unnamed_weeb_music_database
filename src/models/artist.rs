use super::{ExternalSite, Name};
use async_graphql::Object;
use sea_query::Iden;
use sqlx::{postgres::PgRow, FromRow, Row, types::chrono::{DateTime, Utc}, Decode};
use ulid::Ulid;

#[derive(async_graphql::Enum, Clone, Debug, PartialEq, Eq, Copy, Decode)]
pub enum ArtistType {
    /// Indicates that the artist is a single person.
    Solo,
    /// Indicates that the artist is a fictional character.
    Character,
    /// Indicates that the artist is a group of people.
    Group,
    /// Indicates that the artist is an orchestra.
    Orchestra,
    /// Indicates that the artist is a choir.
    Choir,
    /// Anything that is not covered by the other types.
    Other,
}

#[derive(Clone, Debug)]
pub struct Artist {
    /// Unique ID of the artist.
    pub id: Ulid,
    /// Contains the name of the artist.
    pub name: Name,
    /// Contains an array of alternative names.
    pub alt_names: Option<Vec<Name>>,
    /// Contains an array of external links (YouTube, Apple Music and etc)
    pub external_sites: Option<Vec<ExternalSite>>,
    /// Contains a description of the artist.
    pub description: Option<String>,
    /// Contains the location where the artist is based in.
    pub based_in: Option<String>,
    /// Contains the date when the artist was founded.
    pub founded_in: Option<DateTime<Utc>>,
    /// Contains the type of the artist.
    /// 
    /// This is used to determine how to display the artist.
    /// For example, a group of people will be displayed differently than a single person.
    pub artist_type: ArtistType,

    pub join_phrase: Option<String>,
}

impl<'r> FromRow<'r, PgRow> for Artist {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let name: Name = row.try_get("name")?;
        let alt_names: Option<Vec<Name>> = row.try_get("alt_names")?;
        let external_sites: Option<Vec<ExternalSite>> = row.try_get("external_sites")?;
        let description: Option<String> = row.try_get("description")?;
        let based_in: Option<String> = row.try_get("based_in")?;
        let founded_in: Option<DateTime<Utc>> = row.try_get("founded_in")?;
        let artist_type: ArtistType = row.try_get("artist_type")?;
        let join_phrase: Option<String> = row.try_get("join_phrase").unwrap_or(None);

        Ok(Self {
            id: Ulid::from_string(&id).unwrap(),
            name,
            alt_names,
            external_sites,
            description,
            based_in,
            founded_in,
            artist_type,
            join_phrase,
        })
    }
}

pub enum ArtistIden {
    Table,
    Id,
    Name,
    AltNames,
    ExternalSites,
    Description,
    BasedIn,
    FoundedIn,
    ArtistType,
}

impl Iden for ArtistIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                ArtistIden::Table => "artists",
                ArtistIden::Id => "id",
                ArtistIden::Name => "name",
                ArtistIden::AltNames => "alt_names",
                ArtistIden::ExternalSites => "external_sites",
                ArtistIden::Description => "description",
                ArtistIden::BasedIn => "based_in",
                ArtistIden::FoundedIn => "founded_in",
                ArtistIden::ArtistType => "artist_type",
            }
        )
        .unwrap();
    }
}

pub enum SongArtistIden {
    Table,
    ArtistId,
    SongId,
    JoinPhrase
}

impl Iden for SongArtistIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                SongArtistIden::Table => "songs_artists",
                SongArtistIden::ArtistId => "artist_id",
                SongArtistIden::SongId => "song_id",
                SongArtistIden::JoinPhrase => "join_phrase",
            }
        )
        .unwrap();
    }
}

#[Object]
impl Artist {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn name(&self) -> &Name {
        &self.name
    }

    async fn alt_names(&self) -> Option<&Vec<Name>> {
        self.alt_names.as_ref()
    }

    async fn external_sites(&self) -> Option<&Vec<ExternalSite>> {
        self.external_sites.as_ref()
    }

    async fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    async fn based_in(&self) -> Option<&String> {
        self.based_in.as_ref()
    }

    async fn founded_in(&self) -> Option<&DateTime<Utc>> {
        self.founded_in.as_ref()
    }

    async fn artist_type(&self) -> &ArtistType {
        &self.artist_type
    }

    async fn join_phrase(&self) -> Option<&String> {
        self.join_phrase.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct Options {
    pub id: Option<String>,
    pub search: Option<String>,
    pub song_id: Option<String>,
    pub release_id: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

// Implementing sqlx::Type for ArtistType
impl sqlx::Type<sqlx::Postgres> for ArtistType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("artist_type")
    }
}