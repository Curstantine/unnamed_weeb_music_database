use super::{Name, ExternalSite};
use async_graphql::Object;
use sea_query::Iden;
use sqlx::{postgres::PgRow, FromRow, Row};
use ulid::Ulid;

// #[derive(GraphQLEnum)]
// pub enum ArtistType {
//     Unknown,
//     Singer,
//     Producer,
//     Remixer,
// }

#[derive(Clone, Debug)]
pub struct Artist {
    pub id: Ulid,
    pub name: Name,
    pub alt_names: Option<Vec<Name>>,
    /// Contains an array of external links (YouTube, Apple Music and etc)
    pub external_sites: Option<Vec<ExternalSite>>,
    pub description: Option<String>,
    // artist_type: ArtistType,
}

impl<'r> FromRow<'r, PgRow> for Artist {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get(0)?;
        let name: Name = row.try_get(1)?;
        let alt_names: Option<Vec<Name>> = row.try_get(2)?;
        let external_sites: Option<Vec<ExternalSite>> = row.try_get(3)?;
        let description: Option<String> = row.try_get(4)?;

        Ok(Self {
            id: Ulid::from_string(&id).unwrap(),
            name,
            alt_names,
            external_sites,
            description,
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
            }
        )
        .unwrap();
    }
}

pub enum SongArtistIden {
    Table,
    ArtistId,
    SongId,
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
