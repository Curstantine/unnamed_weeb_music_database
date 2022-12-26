pub mod artist;
pub mod release;
pub mod song;

pub mod external_links;

// Re-exporting the external links module.
pub use crate::models::external_links::*;

use std::error::Error;

use async_graphql::{InputObject, Object};

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

// Implementing Decode for Name
//
// This is required for Name to be decoded properly.
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

// Implementing Type for Name
//
// This is required for Name to be decoded properly.
impl sqlx::Type<sqlx::Postgres> for Name {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("localized_name")
    }
}

// Implementing PgHasArrayType for Name
//
// This is required for arrays of Name to be decoded properly.
impl sqlx::postgres::PgHasArrayType for Name {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_localized_name")
    }
}