use crate::{
    models::release::{Release, ReleaseIden, SongReleaseIden},
    utils::error::Error,
};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::PgPool;
use ulid::Ulid;

use crate::sea_query_driver_postgres::bind_query_as;

pub async fn get_releases_by_song_id(id: &Ulid, db: &PgPool) -> Result<Vec<Release>, Error> {
    let (query, values) = Query::select()
        .columns([
            (ReleaseIden::Table, ReleaseIden::Id),
            (ReleaseIden::Table, ReleaseIden::Name),
            (ReleaseIden::Table, ReleaseIden::ReleaseType),
            (ReleaseIden::Table, ReleaseIden::TotalTracks),
        ])
        .from(SongReleaseIden::Table)
        .join(
            sea_query::JoinType::LeftJoin,
            ReleaseIden::Table,
            Expr::col(SongReleaseIden::ReleaseId).equals(ReleaseIden::Table, ReleaseIden::Id),
        )
        .and_where(Expr::col(SongReleaseIden::SongId).eq(id.to_string()))
        .build(PostgresQueryBuilder);

    println!("{}", query);

    let releases: Vec<Release> = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_all(db)
        .await
        .unwrap();

    Ok(releases)
}
