use crate::{
    models::{
        artist::{Artist, ArtistIden, Options, SongArtistIden},
        release::SongReleaseIden,
        song::SongIden,
    },
    utils::error::Error,
};

use sea_query::{Expr, JoinType, PostgresQueryBuilder, Query, Values};
use sqlx::PgPool;
use ulid::Ulid;

use crate::sea_query_driver_postgres::bind_query_as;

pub async fn get_artists_by_song_id(id: &Ulid, db: &PgPool) -> Result<Vec<Artist>, Error> {
    let (query, values) = Query::select()
        .columns([
            (ArtistIden::Table, ArtistIden::Id),
            (ArtistIden::Table, ArtistIden::Name),
        ])
        .from(SongArtistIden::Table)
        .join(
            JoinType::LeftJoin,
            ArtistIden::Table,
            Expr::col(SongArtistIden::ArtistId).equals(ArtistIden::Table, ArtistIden::Id),
        )
        .and_where(Expr::col(SongArtistIden::SongId).eq(id.to_string()))
        .build(PostgresQueryBuilder);

    println!("{}", query);

    let artists: Vec<Artist> = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_all(db)
        .await
        .unwrap();

    Ok(artists)
}

/// get artist with options
/// # Arguments
/// * `options` - options for artist
/// * `db` - database connection
pub async fn get_artist(options: &Options, db: &PgPool) -> Result<Artist, Error> {
    let (query, values) = build_query(options);

    println!("{}", query);

    // execute the query
    let artist: Artist = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_one(db)
        .await
        .unwrap();

    Ok(artist)
}

/// get artists with options
/// # Arguments
/// * `options` - options for artist
/// * `db` - database connection
/// # Returns
/// * `Vec<Artist>` - artists
pub async fn get_artists(options: &Options, db: &PgPool) -> Result<Vec<Artist>, Error> {
    let (query, values) = build_query(options);

    println!("{}", query);

    // execute the query
    let artists: Vec<Artist> = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_all(db)
        .await
        .unwrap();

    Ok(artists)
}

fn build_query(options: &Options) -> (String, Values) {
    let mut q = Query::select();

    q.columns([
        (ArtistIden::Table, ArtistIden::Id),
        (ArtistIden::Table, ArtistIden::Name),
    ]);

    if options.song_id.is_none() && options.release_id.is_none() {
        q.from(ArtistIden::Table);
    } else if options.song_id.is_some() {
        q.from(SongArtistIden::Table);
    } else if options.release_id.is_some() {
        q.from(SongReleaseIden::Table);
    }

    if let Some(id) = &options.id {
        q.and_where(Expr::col(ArtistIden::Id).eq(id.clone()));
    }

    if let Some(search) = &options.search {
        q.and_where(
			Expr::cust_with_values("lower((\"name\").native) ~ lower($1) or lower((\"name\").romanized) ~ lower($1) or lower((\"name\").english) ~ lower($1)", vec![search.clone()])
		);
    }

    if let Some(song_id) = &options.song_id {
        q.join(
            JoinType::LeftJoin,
            SongIden::Table,
            Expr::tbl(SongArtistIden::Table, SongArtistIden::ArtistId)
                .equals(ArtistIden::Table, ArtistIden::Id),
        )
        .and_where(Expr::col(SongArtistIden::SongId).eq(song_id.clone()));
    }

    if let Some(release_id) = &options.release_id {
        q.join(
            JoinType::LeftJoin,
            ArtistIden::Table,
            Expr::tbl(SongReleaseIden::Table, SongReleaseIden::SongId)
                .equals(ArtistIden::Table, ArtistIden::Id),
        )
        .and_where(Expr::col(SongReleaseIden::ReleaseId).eq(release_id.clone()));
    }

    if options.page.is_some() || options.per_page.is_some() {
        q.limit(options.per_page.unwrap_or(50) as u64);
        q.offset(options.page.unwrap_or(0) as u64 * options.per_page.unwrap_or(50) as u64);
    }

    q.build(PostgresQueryBuilder)
}
