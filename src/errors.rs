use actix_web::error::BlockingError;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use derive_more::Display;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use std::fmt;

// Helper error conversion types

#[derive(Debug, Display, PartialEq, Serialize, Deserialize)]
#[display(fmt = "ApiError: {}", detail)]
pub struct ApiError {
    pub status_code: u16,
    pub detail: String,
}

impl ApiError {
    pub fn new_internal(detail: String) -> ApiError {
        error!("Internal server error: {}", &detail);
        ApiError {
            status_code: 500,
            detail: format!("Internal Server Error: {}", detail),
        }
    }
}

impl From<r2d2::Error> for ApiError {
    fn from(error: r2d2::Error) -> ApiError {
        ApiError::new_internal(error.to_string())
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        ApiError::new_internal(error.to_string())
    }
}

impl ResponseError for ApiError {}

impl<E: fmt::Debug + Into<ApiError>> From<BlockingError<E>> for ApiError {
    fn from(error: BlockingError<E>) -> Self {
        match error {
            BlockingError::Canceled => {
                ApiError::new_internal(format!("Internal blocking cancelled error: {:?}", error))
            }
            BlockingError::Error(e) => e.into(),
        }
    }
}

fn convert_to_status_code(status: u16) -> StatusCode {
    match StatusCode::from_u16(status) {
        Ok(status_code) => status_code,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
