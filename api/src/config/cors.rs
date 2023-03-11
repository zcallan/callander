use crate::utils::env::is_dev;
use actix_cors::Cors;
use std::env;

pub fn with_cors() -> Cors {
    match is_dev() {
        true => Cors::permissive(),
        false => {
            let allowed_origin = env::var("CORS_ALLOWED").expect("CORS_ALLOWED must be set");
            Cors::default().allowed_origin(&allowed_origin)
        }
    }
}
