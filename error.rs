use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection,Reply};

#[derive(Error, Debug)]

pun enum Error{
    #[error("wrong creadentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError
}

#[derive(Error, Debug)]

struct ErrorResponse {
    message: String,
    status: String, 
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(e: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found(){
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(e) = err.find::<Error>(){
        match e {
            Error::WrongCredentialsError =>(StatusCode::FORBIDDEN, e.to_string()),
            Error::NoPermissionError => (StatusCode::UNATHORIZED, e.to_string()),
            Error::JWTTokenError =>(StatusCode::UNATHORIZED, e.to_string()),
            Error::JWTTokenCreationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            _=>(StatusCode::BAD::REQUEST, E.to_string()),
        }
    } else if err.find::<warp::reject::MethoNotAllowed>().is_some(){
        (
            StatusCode::METHO_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
        )
    } else {
        eprintIn!("unhandle error {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    let json = warp::reply::json(&ErrorResponse{
        status: code.to_string(),
        message,
    });
    Ok(warp::reply::with_status(json, code))
}

