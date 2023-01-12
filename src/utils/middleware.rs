use super::error::{Error, ErrorResponse};
use crate::{constants, models::user::AccessLevel};
use hyper::{
    header::{self, HeaderValue},
    Body, Request, Response,
};
use jsonwebtoken::{decode, Validation};
use routerify::{ext::RequestExt, RouteError};
use serde::{Deserialize, Serialize};
use std::io;
use tracing::{error};

pub async fn setup_headers(mut req: Response<Body>) -> Result<Response<Body>, io::Error> {
    let headers = req.headers_mut();

    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        constants::ALLOWED_CONTROL_HOSTS.clone(),
    );

    headers.insert(
        header::SERVER,
        HeaderValue::from_static(constants::APP_NAME),
    );

    Ok(req)
}

pub async fn logger(req: Request<Body>) -> Result<Request<Body>, io::Error> {
    info!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub iss: String,
    pub aud: String,
    pub iat: usize,
    pub nbf: usize,
    pub exp: usize,
    pub ulid: String,
    pub access_level: AccessLevel,
    pub sid: String,
}

// Implement an authentication middleware that checks for a valid JWT token in the Authorization header.
// This uses routerify's middleware API.
pub async fn auth(req: Request<Body>) -> Result<Request<Body>, io::Error> {
    // Get the auth key from config and decode the token.
    let config = req.data::<crate::config::Config>().unwrap();
    let auth_key = jsonwebtoken::DecodingKey::from_base64_secret(&config.auth_key).unwrap();

    let auth_head = req.headers().get("Authorization");
    match auth_head {
        Some(_) => {
            let token = req
                .headers()
                .get("Authorization")
                .unwrap()
                .to_str()
                .unwrap()
                .replace("Bearer ", "");

            let validation = Validation::default();
            let claims = decode::<Claims>(&token, &auth_key, &validation).unwrap();

            req.set_context(claims.claims);
        }
        None => {
            return Ok(req);
        }
    }

    Ok(req)
}

pub async fn handle_error(err: RouteError) -> Response<Body> {
    error!("Error occurred while serving a request {err}");

    let err = err.downcast::<Error>().unwrap();
    let json = serde_json::to_string(&ErrorResponse::from(err.clone()));

    Response::builder()
        .status(err.status_code)
        .body(Body::from(json.unwrap()))
        .unwrap()
}
