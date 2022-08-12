use super::error::{Error, ErrorResponse};
use crate::constants;
use hyper::{
    header::{self, HeaderValue},
    Body, Request, Response,
};
use routerify::{ext::RequestExt, RouteError};
use std::io;
use tracing::{error, info};

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

pub async fn handle_error(err: RouteError) -> Response<Body> {
    error!("Error occurred while serving a request {err}");

    let err = err.downcast::<Error>().unwrap();
    let json = serde_json::to_string(&ErrorResponse::from(err.clone()));

    Response::builder()
        .status(err.status_code)
        .body(Body::from(json.unwrap()))
        .unwrap()
}
