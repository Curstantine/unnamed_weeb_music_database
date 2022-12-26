use async_graphql::{Enum, Object};
use std::error::Error;

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, sqlx::Encode, sqlx::Decode)]
pub enum ExternalSiteType {
    AppleMusic,
    YouTube,
    Spotify,
    SoundCloud,
    Twitter,
    Instagram,
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, sqlx::Encode, sqlx::Decode)]
pub enum ExternalType {
    Album,
    Song,
    Artist,
}

#[derive(Clone, Debug)]
pub struct ExternalSite {
    pub site: ExternalSiteType,
    pub id: String,
    pub external_type: ExternalType,
}

#[Object]
impl ExternalSite {
    pub async fn site_type(&self) -> ExternalSiteType {
        self.site
    }

    pub async fn url(&self) -> String {
        // Figure out what the parent type is and return the correct URL
        // based on the site type.
        match self.site {
            ExternalSiteType::AppleMusic => match self.external_type {
                ExternalType::Album => format!("https://music.apple.com/us/album/{}", self.id),
                ExternalType::Song => format!("https://music.apple.com/us/song/{}", self.id),
                ExternalType::Artist => format!("https://music.apple.com/us/artist/{}", self.id),
            },
            ExternalSiteType::YouTube => match self.external_type {
                ExternalType::Album => format!("https://www.youtube.com/playlist?list={}", self.id),
                ExternalType::Song => format!("https://www.youtube.com/watch?v={}", self.id),
                ExternalType::Artist => format!("https://www.youtube.com/channel/{}", self.id),
            },
            ExternalSiteType::Spotify => match self.external_type {
                ExternalType::Album => format!("https://open.spotify.com/album/{}", self.id),
                ExternalType::Song => format!("https://open.spotify.com/track/{}", self.id),
                ExternalType::Artist => format!("https://open.spotify.com/artist/{}", self.id),
            },
            ExternalSiteType::SoundCloud => format!("https://soundcloud.com/{}", self.id),
            ExternalSiteType::Twitter => format!("https://twitter.com/{}", self.id),
            ExternalSiteType::Instagram => format!("https://instagram.com/{}", self.id),
        }
    }
}

// Implementing Decode for ExternalSites
//
// This is required for ExternalSites to be decoded properly.
impl<'r> sqlx::decode::Decode<'r, sqlx::Postgres> for ExternalSite {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let site_type = decoder.try_decode::<ExternalSiteType>()?;
        let site_id = decoder.try_decode::<String>()?;
        let external_type = decoder.try_decode::<ExternalType>()?;
        Ok(ExternalSite {
            site: site_type,
            id: site_id,
            external_type,
        })
    }
}

// Implementing Type for ExternalSites
//
// This is required for ExternalSites to be decoded properly.
impl sqlx::Type<sqlx::Postgres> for ExternalSite {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("external_site")
    }
}

// Implementing Type for ExternalSiteType
//
// This is required for ExternalSiteType to be decoded properly.
impl sqlx::Type<sqlx::Postgres> for ExternalSiteType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("ext_site_type")
    }
}

//Implementing Type for ExternalType
//
// This is required for ExternalType to be decoded properly.
impl sqlx::Type<sqlx::Postgres> for ExternalType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("external_type")
    }
}

// Implementing PgHasArrayType for ExternalSites
//
// This is required for arrays of ExternalSites to be decoded properly.
impl sqlx::postgres::PgHasArrayType for ExternalSite {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_external_site")
    }
}
