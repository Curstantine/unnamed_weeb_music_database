use crate::{
    models::{
        release::{Options, Release, ReleaseIden, SongReleaseIden},
        song::SongIden,
    },
    utils::error::Error,
};
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query};
use sqlx::PgPool;
use ulid::Ulid;

use crate::sea_query_driver_postgres::bind_query_as;

pub async fn get_releases(_options: &Options, db: &PgPool) -> Result<Vec<Release>, Error> {
    let sr: sea_query::DynIden = sea_query::SeaRc::new(sea_query::Alias::new("sr"));
    let s: sea_query::DynIden = sea_query::SeaRc::new(sea_query::Alias::new("s"));
    let total_length: sea_query::DynIden =
        sea_query::SeaRc::new(sea_query::Alias::new("total_length"));
    let (query, values) = Query::select()
        // Get all columns from the release table
        .expr(Expr::table_asterisk(ReleaseIden::Table))
        .expr(Expr::col(total_length.clone()))
        .from(ReleaseIden::Table)
        .join_subquery(
            sea_query::JoinType::LeftJoin,
            // Sum the length of all the songs in the release
            Query::select()
                .column(SongReleaseIden::ReleaseId)
                .expr_as(
                    Expr::col((SongIden::Table, SongIden::TrackLength)).sum(),
                    total_length.clone(),
                )
                .from_as(SongReleaseIden::Table, sr.clone())
                .inner_join(
                    SongIden::Table,
                    Expr::col((sr.clone(), SongReleaseIden::SongId))
                        .equals(SongIden::Table, SongIden::Id),
                )
                .add_group_by(vec![Expr::col(SongReleaseIden::ReleaseId).into()])
                .to_owned()
                .take(),
            s.clone(),
            Expr::col((ReleaseIden::Table, ReleaseIden::Id))
                .equals(s.clone(), SongReleaseIden::ReleaseId),
        )
        .build(PostgresQueryBuilder);

    let releases: Vec<Release> = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_all(db)
        .await
        .unwrap();

    Ok(releases)
}

pub async fn get_releases_by_song_id(id: &Ulid, db: &PgPool) -> Result<Vec<Release>, Error> {
    let (query, values) = Query::select()
        .expr(Expr::table_asterisk(ReleaseIden::Table))
        .expr_as(
            Expr::col((SongIden::Table, SongIden::TrackLength)).sum(),
            Alias::new("total_length"),
        )
        .from(ReleaseIden::Table)
        .left_join(
            SongReleaseIden::Table,
            Expr::col((ReleaseIden::Table, ReleaseIden::Id))
                .equals(SongReleaseIden::Table, SongReleaseIden::ReleaseId),
        )
        .left_join(
            SongIden::Table,
            Expr::col((SongReleaseIden::Table, SongReleaseIden::SongId))
                .equals(SongIden::Table, SongIden::Id),
        )
        .and_where(
            Expr::col((ReleaseIden::Table, ReleaseIden::Id)).in_subquery(
                Query::select()
                    .column(SongReleaseIden::ReleaseId)
                    .from(SongReleaseIden::Table)
                    .and_where(Expr::col(SongReleaseIden::SongId).eq(id.to_string()))
                    .take(),
            ),
        )
        .group_by_col((ReleaseIden::Table, ReleaseIden::Id))
        .build(PostgresQueryBuilder);

    println!("{}", query);

    let releases: Vec<Release> = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_all(db)
        .await
        .unwrap();

    Ok(releases)
}
