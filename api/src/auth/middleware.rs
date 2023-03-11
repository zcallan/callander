use core::fmt;
use std::env;
use std::future::{ready, Ready};
use std::pin::Pin;
use uuid::Uuid;

use actix_web::error::ErrorUnauthorized;
use actix_web::web::BytesMut;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use futures::Future;
use jwksclient2::error::Error;
use jwksclient2::keyset::KeyStore;
use log::info;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub struct JwtMiddleware {
    pub user_id: uuid::Uuid,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let api_url = env::var("HANKO_API_URL").expect("Hanko API url not set");
            let url = api_url.to_owned() + "/.well-known/jwks.json";

            let token = req
                .headers()
                .get(http::header::AUTHORIZATION)
                .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
                .or_else(|| req.cookie("hanko").map(|c| c.value().to_string()));

            if token.is_none() {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "You are not logged in, please provide token".to_string(),
                };
                return Err(ErrorUnauthorized(json_error));
            }

            let key_set = KeyStore::new_from(url.to_string()).await.unwrap();
            let token = token.unwrap();

            match key_set.verify(&token) {
                Ok(jwt) => {
                    let sub = jwt.payload().get_str("sub").unwrap();
                    let user_id = Uuid::try_from(sub).unwrap();

                    return Ok(JwtMiddleware { user_id });
                }
                Err(Error { msg, typ: _ }) => {
                    eprintln!("Could not verify token. Reason: {}", msg);
                    let json_error = ErrorResponse {
                        status: "fail".to_string(),
                        message: "You are not logged in, please provide token".to_string(),
                    };
                    return Err(ErrorUnauthorized(json_error));
                }
            }
        })
    }
}
