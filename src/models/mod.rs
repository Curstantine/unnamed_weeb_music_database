pub mod artist;
pub mod refresh_token;
pub mod release;
pub mod song;
pub mod user;

pub mod external_links;

// Re-exporting the external links module.
pub use crate::models::external_links::*;

use std::error::Error;

use async_graphql::{InputObject, Object};

use sea_query::Value;

#[derive(Clone, Debug, InputObject)]
pub struct NewName {
    pub native: Option<String>,
    pub romanized: Option<String>,
    pub english: Option<String>,
}

#[derive(Clone, Debug, sqlx::Encode)]
pub struct Name {
    /// Native name the original variant uses.
    ///
    /// "残酷な天使のテーゼ"
    pub native: Option<String>,
    /// Romanized variant of the native title.
    ///
    /// "Zankoku na Tenshi no Tēze"
    pub romanized: Option<String>,
    /// English translated name.
    ///
    /// "The Cruel Angel's Thesis"
    pub english: Option<String>,
}

#[Object]
impl Name {
    pub async fn native(&self) -> Option<&str> {
        self.native.as_deref()
    }

    pub async fn romanized(&self) -> Option<&str> {
        self.romanized.as_deref()
    }

    pub async fn english(&self) -> Option<&str> {
        self.english.as_deref()
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
        let native = decoder.try_decode::<Option<String>>()?;
        let romanized = decoder.try_decode::<Option<String>>()?;
        let english = decoder.try_decode::<Option<String>>()?;
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

// Implementing From for Name to Value for sea_query
// This is required for sea_query to be able to construct queries with Name easily.
// This requires sea_query to have the "array" feature enabled.
impl From<Name> for sea_query::Value {
    fn from(name: Name) -> Self {
        Value::Array(Some(Box::new(vec![
            Value::from(name.native),
            Value::from(name.romanized),
            Value::from(name.english),
        ])))
    }
}
