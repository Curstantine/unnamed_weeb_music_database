use crate::{
    models::{
        artist::SongArtistIden,
        release::SongReleaseIden,
        song::{Options, Song, SongIden},
        Name,
    },
    utils::error::Error,
};
use sea_query::{Expr, JoinType, PostgresQueryBuilder, Query, Values};
use sqlx::PgPool;

use crate::sea_query_driver_postgres::bind_query_as;

/// Returns a song with the given id from the database.
///
/// # Arguments
/// * `Options` - Options deciding what song to be returned.
/// # Errors
/// * `Error::NotFound` - If the song with the given id does not exist.
/// # Returns
/// * `Song` - The song with the given id.
pub async fn get_song(options: &Options, db: &PgPool) -> Result<Song, Error> {
    let (query, values) = build_query(options);

    println!("{}", query);

    let song: Song = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_one(db)
        .await
        .unwrap();

    Ok(song)
}

/// Returns a song with the given id from the database.
///
/// # Arguments
/// * `Options` - Options deciding what song to be returned.
/// # Errors
/// * `Error::NotFound` - If the song with the given id does not exist.
/// # Returns
/// * `Song` - The song with the given id.
pub async fn get_songs(options: &Options, db: &PgPool) -> Result<Vec<Song>, Error> {
    let (query, values) = build_query(options);

    println!("{}", query);

    let songs: Vec<Song> = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_all(db)
        .await
        .unwrap();

    Ok(songs)
}

pub async fn create_song(
    ulid: ulid::Ulid,
    name: Name,
    _artists: Vec<String>,
    _releases: Option<Vec<String>>,
    db: &PgPool,
) -> Result<Song, Error> {
    let (query, values) = Query::insert()
        .into_table(SongIden::Table)
        .columns([SongIden::Id, SongIden::Name])
        .values_panic(vec![
            ulid.to_string().into(),
            vec![
                name.native.clone(),
                name.romanized.clone(),
                name.english.clone(),
            ]
            .into(),
        ])
        .returning(Query::returning().columns([SongIden::Id, SongIden::Name]))
        .build(PostgresQueryBuilder);

    println!("{}", query);
    println!("{:?}", values);

    let song: Song = sqlx::query_as(&query)
        .bind(ulid.to_string())
        .bind(name)
        .fetch_one(db)
        .await
        .unwrap();

    Ok(song)
}

fn build_query(options: &Options) -> (String, Values) {
    let mut q = Query::select();

    q.columns([
        (SongIden::Table, SongIden::Id),
        (SongIden::Table, SongIden::Name),
    ]);

    if options.artist_id.is_none() && options.release_id.is_none() {
        q.from(SongIden::Table);
    } else if options.artist_id.is_some() {
        q.from(SongArtistIden::Table);
    } else if options.release_id.is_some() {
        q.from(SongReleaseIden::Table);
    }

    if let Some(id) = &options.id {
        q.and_where(Expr::col(SongIden::Id).eq(id.clone()));
    }

    if let Some(search) = &options.search {
        q.and_where(
			Expr::cust_with_values("lower((\"name\").native) ~ lower($1) or lower((\"name\").romanized) ~ lower($1) or lower((\"name\").english) ~ lower($1)", vec![search.clone()])
		);
    }

    if let Some(artist_id) = &options.artist_id {
        q.join(
            JoinType::LeftJoin,
            SongIden::Table,
            Expr::tbl(SongArtistIden::Table, SongArtistIden::SongId)
                .equals(SongIden::Table, SongIden::Id),
        )
        .and_where(Expr::col(SongArtistIden::ArtistId).eq(artist_id.clone()));
    }

    if let Some(release_id) = &options.release_id {
        q.join(
            JoinType::LeftJoin,
            SongIden::Table,
            Expr::tbl(SongReleaseIden::Table, SongReleaseIden::SongId)
                .equals(SongIden::Table, SongIden::Id),
        )
        .and_where(Expr::col(SongReleaseIden::ReleaseId).eq(release_id.clone()));
    }

    if options.page.is_some() || options.per_page.is_some() {
        q.limit(options.per_page.unwrap_or(50) as u64);
        q.offset(options.page.unwrap_or(0) as u64 * options.per_page.unwrap_or(50) as u64);
    }

    q.build(PostgresQueryBuilder)
}
