use diesel::result::{Error, DatabaseErrorKind};
use rocket::http::Status;

pub fn err_to_response(error: Error) -> (Status, String) {
    let status = match error {
        Error::NotFound => Status::NotFound,
        Error::QueryBuilderError(_) => Status::UnprocessableEntity,
        Error::DatabaseError(kind, ref info) => {
            match kind {
                DatabaseErrorKind::UniqueViolation => Status::Conflict,
                DatabaseErrorKind::ForeignKeyViolation => Status::UnprocessableEntity,
                _ => {
                    match info.constraint_name() {
                        Some(_) => Status::UnprocessableEntity,
                        None => Status::InternalServerError
                    }
                }
            }
        }
        _ => Status::InternalServerError,
    };
    
    (status, format!("{}: {}", status.code.to_string(), error.to_string()))
}
