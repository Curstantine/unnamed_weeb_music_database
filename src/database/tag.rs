use sea_query::{Expr, JoinType, PostgresQueryBuilder, Query, Values};
use sqlx::PgPool;

use crate::{
    models::tag::{Options, ReleaseTagIden, SongTagIden, Tag, TagIden},
    sea_query_driver_postgres::bind_query_as,
    utils::error::Error,
};

pub async fn get_tags(options: &Options, db: &PgPool) -> Result<Vec<Tag>, Error> {
    let (query, values) = build_query(options);

    let tag: Vec<Tag> = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_all(db)
        .await?;

    Ok(tag)
}

fn build_query(options: &Options) -> (String, Values) {
    let mut q = Query::select();
    q.expr(Expr::table_asterisk(TagIden::Table));
    q.from(TagIden::Table);

    if options.id.is_some() {
        q.and_where(Expr::col(TagIden::Id).eq(options.id.as_ref().unwrap().to_string()));
    } else if options.name.is_some() {
        q.and_where(Expr::col(TagIden::Name).eq(options.name.as_ref().unwrap().clone()));
    } else if options.song_id.is_some() {
        q.expr(Expr::col(SongTagIden::SongId));
        q.join(
            JoinType::LeftJoin,
            SongTagIden::Table,
            Expr::tbl(TagIden::Table, TagIden::Id).equals(SongTagIden::Table, SongTagIden::TagId),
        );
        q.and_where(
            Expr::col(SongTagIden::SongId).eq(options.song_id.as_ref().unwrap().to_string()),
        );
    } else if options.release_id.is_some() {
        q.expr(Expr::col(ReleaseTagIden::ReleaseId));
        q.join(
            JoinType::LeftJoin,
            ReleaseTagIden::Table,
            Expr::tbl(TagIden::Table, TagIden::Id)
                .equals(ReleaseTagIden::Table, ReleaseTagIden::TagId),
        );
        q.and_where(
            Expr::col(ReleaseTagIden::ReleaseId).eq(options
                .release_id
                .as_ref()
                .unwrap()
                .to_string()),
        );
    }

    q.to_owned().build(PostgresQueryBuilder)
}

// Tests for building the query
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::tag::Options;

    #[test]
    fn test_build_query() {
        let options = Options {
            id: Some(0),
            name: None,
            song_id: None,
            release_id: None,
        };
        let (query, values) = build_query(&options);
        assert_eq!(
            query.replace('\"', ""),
            "SELECT tags.* FROM tags WHERE id = $1"
        );
        assert_eq!(values.0, vec!["0".into()]);
    }

    #[test]
    fn test_build_query_with_name() {
        let options = Options {
            id: None,
            name: Some("test".to_string()),
            song_id: None,
            release_id: None,
        };
        let (query, values) = build_query(&options);
        assert_eq!(
            query.replace('\"', ""),
            "SELECT tags.* FROM tags WHERE name = $1"
        );
        assert_eq!(values.0, vec!["test".into()]);
    }

    #[test]
    fn test_build_query_with_song_id() {
        let options = Options {
            id: None,
            name: None,
            song_id: Some("00000000000000000000000000".parse().unwrap()),
            release_id: None,
        };
        let (query, values) = build_query(&options);
        assert_eq!(query.replace('\"', ""), "SELECT tags.*, song_id FROM tags LEFT JOIN song_tags ON tags.id = song_tags.tag_id WHERE song_id = $1");
        assert_eq!(values.0, vec!["00000000000000000000000000".into()]);
    }

    #[test]
    fn test_build_query_with_release_id() {
        let options = Options {
            id: None,
            name: None,
            song_id: None,
            release_id: Some("00000000000000000000000000".parse().unwrap()),
        };
        let (query, values) = build_query(&options);
        assert_eq!(query.replace('\"', ""), "SELECT tags.*, release_id FROM tags LEFT JOIN release_tags ON tags.id = release_tags.tag_id WHERE release_id = $1");
        assert_eq!(values.0, vec!["00000000000000000000000000".into()]);
    }
}
