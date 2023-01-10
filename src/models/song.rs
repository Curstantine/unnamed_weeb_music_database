use super::{artist::Artist, release::Release, ExternalSite, Name, NewName};
use async_graphql::{Context, InputObject, Object};
use sea_query::Iden;
use sqlx::{postgres::PgRow, types::chrono::NaiveDate, FromRow, PgPool, Row};
use ulid::Ulid;

#[derive(Clone, Debug)]

pub struct Song {
    pub id: Ulid,
    pub name: Name,
    pub external_sites: Option<Vec<ExternalSite>>,
    pub track_length: Option<i32>,
    pub release_date: Option<NaiveDate>,
}

#[allow(dead_code)]
pub enum SongIden {
    Table,
    Id,
    Name,
    TrackLength,
    ExternalSites,
    ReleaseDate,
}

impl Iden for SongIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                SongIden::Table => "songs",
                SongIden::Id => "id",
                SongIden::Name => "name",
                SongIden::TrackLength => "track_length",
                SongIden::ExternalSites => "external_sites",
                SongIden::ReleaseDate => "release_date",
            }
        )
        .unwrap();
    }
}

#[Object]
impl Song {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn name(&self) -> &Name {
        &self.name
    }

    async fn artists<'ctx>(&self, context: &Context<'ctx>) -> Vec<Artist> {
        let db = context.data_unchecked::<PgPool>();
        crate::database::artist::get_artists_by_song_id(&self.id, db)
            .await
            .unwrap()
    }

    async fn releases<'ctx>(&self, context: &Context<'ctx>) -> Vec<Release> {
        let db = context.data_unchecked::<PgPool>();
        crate::database::release::get_releases_by_song_id(&self.id, db)
            .await
            .unwrap()
    }

    async fn external_sites(&self) -> Option<&Vec<ExternalSite>> {
        self.external_sites.as_ref()
    }

    async fn track_length(&self) -> Option<&i32> {
        self.track_length.as_ref()
    }

    async fn release_date(&self) -> Option<&NaiveDate> {
        self.release_date.as_ref()
    }
}

impl<'r> FromRow<'r, PgRow> for Song {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let name: Name = row.try_get("name")?;
        let external_sites: Option<Vec<ExternalSite>> = row.try_get("external_sites")?;
        let track_length: Option<i32> = row.try_get("track_length")?;
        let release_date: Option<NaiveDate> = row.try_get("release_date")?;

        Ok(Self {
            id: Ulid::from_string(&id).unwrap(),
            name,
            external_sites,
            track_length,
            release_date,
        })
    }
}

#[derive(Clone, Debug, InputObject)]
pub struct NewSong {
    pub name: NewName,
    pub artists: Vec<String>,
    pub releases: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub id: Option<String>,
    pub search: Option<String>,
    pub artist_id: Option<String>,
    pub release_id: Option<String>,
    pub genres: Option<Vec<String>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}
