use std::io::Cursor;
use rocket::{Request, response, Response};
use rocket::http::Status;
use rocket::response::Responder;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    // kitchen errors
    #[error("Invalid token")]
    InvalidToken,
    #[error("No file uploaded")]
    NoFileUploaded,
}

impl ApiError {
    pub fn status_code(&self) -> Status {
        match self {
            ApiError::InvalidToken => Status::Unauthorized,
            ApiError::NoFileUploaded => Status::BadRequest,
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'o> {
        let status = self.status_code();
        let body = json!({
            "status": status.code,
            "message": self.to_string(),
        }).to_string();


        Response::build()
            .sized_body(body.len(), Cursor::new(body))
            .status(status)
            .ok()
    }
}
