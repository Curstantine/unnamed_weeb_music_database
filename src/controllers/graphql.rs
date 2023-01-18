use crate::{
    constants::AUTH_DEFAULT_ACCESS_LEVEL,
    controllers::page::{Page, PageInfo},
    database::user::LoginResponse,
    models::{
        artist::Artist,
        refresh_token::{RefreshTokenInput, RefreshedToken},
        song::{NewSong, Song},
        user::{Login, Register, User},
        Name,
    },
    utils::{error::Error, middleware::Claims},
};
use async_graphql::{http::graphiql_source, Context, EmptySubscription, Object, Schema};
use hyper::{Body, Request, Response};
use routerify::prelude::*;
use sqlx::PgPool;
use std::{io, sync::Arc};

pub async fn graphiql(_: Request<Body>) -> Result<Response<Body>, io::Error> {
    let html = graphiql_source("/graphql", None);
    Ok(Response::new(Body::from(html)))
}

pub async fn graphql(req: Request<Body>) -> Result<Response<Body>, io::Error> {
    let schema = &*req
        .data::<Arc<Schema<QueryRoot, MutationRoot, EmptySubscription>>>()
        .unwrap()
        .clone();
    let db = req.data::<PgPool>().unwrap().clone();
    let claims = req.context::<Claims>();
    let mut request = deserialize_body(req.into_body()).await?;
    if claims.is_some() {
        request = request.data(claims.unwrap());
    }
    let response = schema.execute(request.data(db)).await;

    Ok(Response::new(Body::from(
        serde_json::to_string(&response).unwrap(),
    )))
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn song<'ctx>(
        &self,
        context: &Context<'ctx>,
        id: Option<String>,
        search: Option<String>,
        artist_id: Option<String>,
        release_id: Option<String>,
        genres: Option<Vec<String>>,
    ) -> Result<Song, Error> {
        // Ok(Song)
        let db = context.data_unchecked::<PgPool>();
        let mut options = crate::models::song::Options {
            id: None,
            search: None,
            artist_id: None,
            release_id: None,
            genres: None,
            page: None,
            per_page: None,
        };

        if let Some(id) = id {
            options.id = Some(id);
        }

        if let Some(search) = search {
            options.search = Some(search);
        }

        if let Some(artist_id) = artist_id {
            options.artist_id = Some(artist_id);
        }

        if let Some(release_id) = release_id {
            options.release_id = Some(release_id);
        }

        if let Some(genres) = genres {
            options.genres = Some(genres);
        }

        crate::database::song::get_song(&options, db).await
    }

    async fn artist<'ctx>(
        &self,
        context: &Context<'ctx>,
        id: Option<String>,
        search: Option<String>,
        song_id: Option<String>,
        release_id: Option<String>,
    ) -> Result<Artist, Error> {
        // Ok(Artist)
        let db = context.data_unchecked::<PgPool>();
        let mut options = crate::models::artist::Options {
            id: None,
            search: None,
            song_id: None,
            release_id: None,
            page: None,
            per_page: None,
        };

        if let Some(id) = id {
            options.id = Some(id);
        }

        if let Some(search) = search {
            options.search = Some(search);
        }

        if let Some(song_id) = song_id {
            options.song_id = Some(song_id);
        }

        if let Some(release_id) = release_id {
            options.release_id = Some(release_id);
        }

        crate::database::artist::get_artist(&options, db).await
    }

    async fn page<'ctx>(&self, page: Option<i32>, per_page: Option<i32>) -> Result<Page, Error> {
        let page_info = PageInfo {
            total: 0,
            per_page,
            current_page: page,
            last_page: 0,
        };

        /*println!("{:?}", page_info);
        let s = executor.location();
        println!("{:?}", s);*/

        // Return an error if there are two different fields in the look_ahead.
        /* let look_ahead = executor.look_ahead();
        let query_typed = look_ahead.children();
        for child in query_typed {
            println!("{:?}", child);
        } */
        //println!("{:?}", page_children);

        Ok(Page { page_info })
    }

    async fn user<'ctx>(
        &self,
        context: &Context<'ctx>,
        id: Option<String>,
        email: Option<String>,
    ) -> Result<User, Error> {
        // Ok(User)
        let db = context.data_unchecked::<PgPool>();
        let mut options = crate::models::user::Options {
            id: None,
            email: None,
            page: None,
            per_page: None,
        };

        if let Some(id) = id {
            options.id = Some(id);
        }

        if let Some(email) = email {
            options.email = Some(email);
        }

        crate::database::user::get_user(&options, db).await
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_song<'a>(&self, context: &Context<'a>, input: NewSong) -> Result<Song, Error> {
        // Ok(Song)
        let db = context.data_unchecked::<PgPool>();
        let ulid = ulid::Ulid::new();
        let name = Name {
            native: input.name.native,
            romanized: input.name.romanized,
            english: input.name.english,
        };
        let artists = input.artists;
        let releases = input.releases;

        crate::database::song::create_song(ulid, name, artists, Some(releases), db).await
        //todo!()
    }

    async fn login<'a>(&self, context: &Context<'a>, input: Login) -> Result<LoginResponse, Error> {
        let db = context.data_unchecked::<PgPool>();
        let password = input.password;
        let email = input.email;
        let username = input.username;

        crate::database::user::login(email, username, password, db).await
    }

    async fn register<'a>(&self, context: &Context<'a>, input: Register) -> Result<User, Error> {
        let db = context.data_unchecked::<PgPool>();
        let email = input.email;
        let password = input.password;
        let username = input.username;

        crate::database::user::create_user(email, username, password, AUTH_DEFAULT_ACCESS_LEVEL, db)
            .await
    }

    async fn refresh_token(
        &self,
        context: &Context<'_>,
        input: RefreshTokenInput,
    ) -> Result<RefreshedToken, Error> {
        let db = context.data_unchecked::<PgPool>();
        let refresh_token = input.token;
        crate::database::user::refresh_token(refresh_token, db).await
    }
}

pub fn make_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot {}, MutationRoot, EmptySubscription).finish()
}

async fn deserialize_body(body: Body) -> Result<async_graphql::Request, io::Error> {
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    // Set the options for the request.
    let options = async_graphql::http::MultipartOptions::default();

    // Convert the bytes into a cursor. To get an implementation of AsyncRead.
    let reader = futures::io::Cursor::new(bytes);

    let req = async_graphql::http::receive_body(Some("application/json"), reader, options).await;

    req.map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}
