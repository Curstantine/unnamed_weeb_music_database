use sea_query::{Alias, Expr, Func, PostgresQueryBuilder, Query};
use sqlx::PgPool;
use ulid::Ulid;

use crate::{
    constants::{APP_NAME, AUTH_DEFAULT_REFRESH_TOKEN_EXPIRATION, JWT_DEFAULT_EXPIRATION},
    models::{
        refresh_token::{RefreshToken, RefreshTokenIden, RefreshedToken},
        user::{AccessLevel, User, UserIden},
    },
    sea_query_driver_postgres::bind_query_as,
    utils::{config::get_config, error::Error},
};

#[derive(Debug, Clone, async_graphql::SimpleObject)]
pub struct LoginToken {
    pub token: String,
    pub refresh_token: String,
}

pub async fn login(
    email: Option<String>,
    username: Option<String>,
    password: String,
    db: &PgPool,
) -> Result<LoginToken, Error> {
    let mut q = Query::select();
    q.expr(Expr::asterisk());
    q.from(UserIden::Table);
    if email.is_some() {
        q.and_where(Expr::col(UserIden::Email).eq(email.unwrap()));
    } else if username.is_some() {
        q.and_where(Expr::col(UserIden::Username).eq(username.unwrap()));
    } else {
        return Err(Error::new("UNAUTHORIZED", hyper::StatusCode::UNAUTHORIZED));
    }
    let (query, values) = q.to_owned().build(PostgresQueryBuilder);

    let user: User = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_one(db)
        .await
        .unwrap();

    println!("Fetched user from database: {:?}", user.id);

    match bcrypt::verify(&password, &user.password_hash) {
        Ok(result) => result,
        Err(_) => return Err(Error::new("UNAUTHORIZED", hyper::StatusCode::UNAUTHORIZED)),
    };

    let token = create_token(user.clone())?;
    let refresh_token = create_refresh_token(user.id, db).await?;

    Ok(LoginToken {
        token,
        refresh_token,
    })
}

fn create_token(user: User) -> Result<String, Error> {
    let conf = get_config();

    let login_claim = crate::utils::middleware::Claims {
        iss: APP_NAME.to_string(),
        aud: APP_NAME.to_string(),
        iat: chrono::Utc::now().timestamp() as usize,
        nbf: chrono::Utc::now().timestamp() as usize,
        exp: (chrono::Utc::now() + chrono::Duration::seconds(JWT_DEFAULT_EXPIRATION as i64))
            .timestamp() as usize,
        ulid: user.id.to_string(),
        access_level: user.access_level,
        //Session ID
        sid: "".to_string(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &login_claim,
        &jsonwebtoken::EncodingKey::from_base64_secret(&conf.auth_key).unwrap(),
    )
    .unwrap();

    Ok(token)
}

async fn create_refresh_token(user_id: Ulid, db: &PgPool) -> Result<String, Error> {
    // Randomly generate a token
    let token = ulid::Ulid::new().to_string();

    // Insert the token into the database with the user id in the refresh_token table
    let (query, values) = Query::insert()
        .into_table(RefreshTokenIden::Table)
        .columns(vec![
            RefreshTokenIden::UserId,
            RefreshTokenIden::Token,
            RefreshTokenIden::ExpiresAt,
        ])
        .values_panic(vec![
            user_id.to_string().into(),
            token.clone().into(),
            (chrono::Utc::now()
                + chrono::Duration::minutes(AUTH_DEFAULT_REFRESH_TOKEN_EXPIRATION as i64))
            .into(),
        ])
        .to_owned()
        .build(PostgresQueryBuilder);

    let _refresh_token: Option<RefreshToken> = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_optional(db)
        .await
        .unwrap();

    Ok(token)
}

pub async fn create_user(
    email: String,
    username: String,
    password: String,
    access_level: AccessLevel,
    db: &PgPool,
) -> Result<User, Error> {
    // Check if the email or username is already taken
    let (query, values) = Query::select()
        .expr(Expr::asterisk())
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Email).eq(email.clone()))
        .cond_where(Expr::col(UserIden::Username).eq(username.clone()))
        .to_owned()
        .build(PostgresQueryBuilder);

    println!("Query: {}", query);

    let user: Option<User> = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_optional(db)
        .await
        .unwrap();

    if user.is_some() {
        return Err(Error::new(
            "USER_ALREADY_EXISTS",
            hyper::StatusCode::BAD_REQUEST,
        ));
    }

    let user = User::new(username, email, password, access_level);
    let (query, values) = Query::insert()
        .into_table(UserIden::Table)
        .columns(vec![
            UserIden::Id,
            UserIden::Email,
            UserIden::Username,
            UserIden::PasswordHash,
            UserIden::AccessLevel,
            UserIden::CreatedAt,
            UserIden::UpdatedAt,
        ])
        .exprs(vec![
            Expr::val(user.id.to_string()).into(),
            Expr::val(user.email.clone()).into(),
            Expr::val(user.username.clone()).into(),
            Expr::val(user.password_hash).into(),
            Func::cast_as(user.access_level, Alias::new("Access_Level")),
            Expr::val(user.created_at).into(),
            Expr::val(user.updated_at).into(),
        ])
        .unwrap()
        .returning_all()
        .to_owned()
        .build(PostgresQueryBuilder);

    println!("Query: {}", query);

    let user: User = bind_query_as(sqlx::query_as(&query), &values)
        .fetch_one(db)
        .await
        .unwrap();

    Ok(user)
}

pub async fn get_user(options: &crate::models::user::Options, db: &PgPool) -> Result<User, Error> {
	let mut q = Query::select();
	q.expr(Expr::asterisk());
	q.from(UserIden::Table);
	if options.id.is_some() {
		q.and_where(Expr::col(UserIden::Id).eq(options.id.as_ref().unwrap().to_string()));
	} else if options.email.is_some() {
		q.and_where(Expr::col(UserIden::Email).eq(options.email.as_ref().unwrap().clone()));
	}
	let (query, values) = q.build(PostgresQueryBuilder);

	let user: Option<User> = bind_query_as(sqlx::query_as(&query), &values)
		.fetch_optional(db)
		.await
		.unwrap();

	if user.is_none() {
		return Err(Error::new("USER_NOT_FOUND", hyper::StatusCode::NOT_FOUND));
	}

	Ok(user.unwrap())
}

pub async fn refresh_token(
	refresh_token: String,
	db: &PgPool,
) -> Result<RefreshedToken, Error> {
	let (query, values) = Query::select()
		.expr(Expr::asterisk())
		.from(RefreshTokenIden::Table)
		.and_where(Expr::col(RefreshTokenIden::Token).eq(refresh_token.clone()))
		.to_owned()
		.build(PostgresQueryBuilder);

	let refresh_token: Option<RefreshToken> = bind_query_as(sqlx::query_as(&query), &values)
		.fetch_optional(db)
		.await
		.unwrap();

	if refresh_token.is_none() {
		return Err(Error::new(
			"REFRESH_TOKEN_NOT_FOUND",
			hyper::StatusCode::NOT_FOUND,
		));
	}

	let refresh_token = refresh_token.unwrap();

	if refresh_token.expires_at < chrono::Utc::now() {
		return Err(Error::new(
			"REFRESH_TOKEN_EXPIRED",
			hyper::StatusCode::UNAUTHORIZED,
		));
	}

	// Update the refresh token
	let (query, values) = Query::update()
		.table(RefreshTokenIden::Table)
		.value(RefreshTokenIden::ExpiresAt, (chrono::Utc::now() + chrono::Duration::seconds(AUTH_DEFAULT_REFRESH_TOKEN_EXPIRATION as i64)).into())
		.and_where(Expr::col(RefreshTokenIden::Token).eq(refresh_token.token.clone()))
		.to_owned()
		.build(PostgresQueryBuilder);

	let _refresh_token: Option<RefreshToken> = bind_query_as(sqlx::query_as(&query), &values)
		.fetch_optional(db)
		.await
		.unwrap();

    let user = get_user(&crate::models::user::Options {
        id: Some(refresh_token.user_id),
        email: None,
        page: None,
        per_page: None,
    }, db).await.unwrap();

	// Create a new jwt token'
	let jwt_token = create_token(user).unwrap();

    // construct RefreshedToken
    let token = RefreshedToken {
        token: jwt_token,
    };
    
	Ok(token)
}