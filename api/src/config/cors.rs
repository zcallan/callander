use crate::utils::env::is_dev;
use actix_cors::Cors;
use std::env;

pub fn with_cors() -> Cors {
    match is_dev() {
        true => Cors::permissive(),
        false => Cors::default().allowed_origin(
            &*env::var("CORS_ALLOWED").expect("Cors allowed_origin list is not set"),
        ),
    }
}
