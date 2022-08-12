pub mod artist;
pub mod release;
pub mod song;

use std::error::Error;

use async_graphql::{Enum, InputObject, Object};

#[derive(Clone, Debug, InputObject)]
pub struct NewName {
    pub native: String,
    pub romanized: String,
    pub english: String,
}

#[derive(Clone, Debug, sqlx::Encode)]
pub struct Name {
    /// Native name the original variant uses.
    ///
    /// "残酷な天使のテーゼ"
    pub native: String,
    /// Romanized variant of the native title.
    ///
    /// "Zankoku na Tenshi no Tēze"
    pub romanized: String,
    /// English translated name.
    ///
    /// "The Cruel Angel's Thesis"
    pub english: String,
}

#[Object]
impl Name {
    pub async fn native(&self) -> &str {
        &self.native
    }

    pub async fn romanized(&self) -> &str {
        &self.romanized
    }

    pub async fn english(&self) -> &str {
        &self.english
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum ExternalSite {
    AppleMusic,
    YouTube,
    Spotify,
}

#[derive(Clone, Debug)]
pub struct ExternalSites {
    site_type: ExternalSite,
    url: String,
}

#[Object]
impl ExternalSites {
    pub async fn site_type(&self) -> ExternalSite {
        self.site_type
    }

    pub async fn url(&self) -> String {
        self.url.clone()
    }
}

impl<'r> sqlx::decode::Decode<'r, sqlx::Postgres> for Name {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let native = decoder.try_decode::<String>()?;
        let romanized = decoder.try_decode::<String>()?;
        let english = decoder.try_decode::<String>()?;
        Ok(Name {
            native,
            romanized,
            english,
        })
    }
}

impl sqlx::Type<sqlx::Postgres> for Name {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("localized_name")
    }
}
