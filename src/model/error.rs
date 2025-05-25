use axum::{
    extract::FromRequest,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Error))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[derive(Debug)]
pub enum Error {
    InternalServerError,
    Forbidden,
    BadRequest(Option<String>),
    EmailDoesNotExist,
    Unauthorized,
    SerdeJsonError(serde_json::Error),
    Argon2Error(argon2::Error),
    AxumError(axum::Error),
    CLapError(clap::Error),
    FileError(std::io::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let res: (StatusCode, String) = (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_owned(),
        );

        let (status, message) = match self {
            Error::Forbidden => (StatusCode::FORBIDDEN, String::from("")),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, String::from("")),
            Error::InternalServerError => {
                tracing::error!("Just Internal");
                res
            }
            Error::BadRequest(m) => {
                tracing::error!("Bad Request");
                (StatusCode::BAD_REQUEST, m.unwrap_or("Unknown".to_string()))
            }
            Error::EmailDoesNotExist => {
                (StatusCode::CONFLICT, String::from("Email does not exist"))
            }
            Error::SerdeJsonError(_error) => res,
            Error::Argon2Error(_error) => res,
            Error::AxumError(_error) => res,
            Error::CLapError(_error) => res,
            Error::FileError(error) => {
                tracing::error!(%error, "error from time_library");
                res
            }
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeJsonError(error)
    }
}

impl From<argon2::Error> for Error {
    fn from(error: argon2::Error) -> Self {
        Self::Argon2Error(error)
    }
}

impl From<axum::Error> for Error {
    fn from(error: axum::Error) -> Self {
        Self::AxumError(error)
    }
}

impl From<clap::Error> for Error {
    fn from(error: clap::Error) -> Self {
        Self::CLapError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::FileError(error)
    }
}
