use super::Name;
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
    ///// Contains an array of external links (YouTube, Apple Music and etc)
    //pub external_sites: Option<Vec<ExternalSites>>,
    // artist_type: ArtistType, <- TODO: Find a better way to handle artist types without trying to duplicate the source.
}

impl<'r> FromRow<'r, PgRow> for Artist {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get(0)?;
        let name: Name = row.try_get(1)?;

        Ok(Self {
            id: Ulid::from_string(&id).unwrap(),
            name,
        })
    }
}

pub enum ArtistIden {
    Table,
    Id,
    Name,
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
