use diesel::result::{Error, DatabaseErrorKind};
use rocket::http::Status;

pub fn err_to_response(error: Error) -> (Status, String) {
    let status: Status;
    let mut msg: &str = "";
    match error {
        Error::NotFound => {
            status = Status::NotFound;
        }
        Error::QueryBuilderError(_) => {
            status = Status::UnprocessableEntity;
            msg = "The query could not be constructed. \nAn example of when this error could occur is if you are attempting to construct an update statement with no changes (e.g. all fields on the struct are None).";
        }
        Error::DatabaseError(kind, ref info) => {
            match kind {
                DatabaseErrorKind::UniqueViolation => {
                    status = Status::Conflict;
                    msg = "A unique constraint was violated.";
                }
                DatabaseErrorKind::ForeignKeyViolation => {
                    status = Status::UnprocessableEntity;
                    msg = "A foreign key constraint was violated.";
                }
                _ => {
                    match info.constraint_name() {
                        Some(constraint_name) => {
                            status = Status::UnprocessableEntity;
                            msg = constraint_name;
                        }
                        None => {
                            status = Status::InternalServerError;
                        }
                    }
                }
            }
        }
        _ => {
            status = Status::InternalServerError;
        }
    };
    
    (status, format!("{} {}\n{}", status.code.to_string(), status.reason().unwrap_or(""), msg))
}
