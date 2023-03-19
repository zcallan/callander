use core::fmt;
use jsonwebtoken::jwk::AlgorithmParameters;
use jsonwebtoken::{decode, decode_header, jwk, DecodingKey, Validation};
use reqwest::Client;
use reqwest_middleware::ClientWithMiddleware;
use std::collections::HashMap;
use std::env;
use std::pin::Pin;

use actix_web::error::ErrorUnauthorized;
use actix_web::web::Data;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, FromRequest, HttpRequest};
use futures::Future;
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
    pub user_id: String,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let api_url = env::var("HANKO_API_URL").expect("Hanko API url not set");
            let url = api_url.to_owned() + "/.well-known/jwks.json";

            let client = req.app_data::<Data<ClientWithMiddleware>>().expect("Wtf");

            // TODO: Cleaner approach to error handling
            let api_response = client
                .get(url)
                .send()
                .await
                .expect("Error fetching jwks")
                .text()
                .await
                .expect("Error parsing jwks");

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

            let token = token.unwrap();

            let header = match decode_header(&token) {
                Ok(h) => h,
                Err(_) => {
                    let json_error = ErrorResponse {
                        status: "fail".to_string(),
                        message: "Token is invalid".to_string(),
                    };
                    return Err(ErrorUnauthorized(json_error));
                }
            };

            let kid = match header.kid {
                Some(k) => k,
                None => {
                    let json_error = ErrorResponse {
                        status: "fail".to_string(),
                        message: "Token doesn't have a `kid` header field".to_string(),
                    };
                    return Err(ErrorUnauthorized(json_error));
                }
            };

            let jwks: jwk::JwkSet = match serde_json::from_str(&api_response) {
                Ok(j) => j,
                Err(_) => {
                    let json_error = ErrorResponse {
                        status: "fail".to_string(),
                        message: "Failed to parse JWKS".to_string(),
                    };
                    return Err(ErrorUnauthorized(json_error));
                }
            };

            if let Some(j) = jwks.find(&kid) {
                match &j.algorithm {
                    AlgorithmParameters::RSA(rsa) => {
                        let decoding_key =
                            DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
                        let mut validation = Validation::new(j.common.algorithm.unwrap());
                        validation.validate_exp = false;
                        let decoded_token = decode::<HashMap<String, serde_json::Value>>(
                            &token,
                            &decoding_key,
                            &validation,
                        )
                        .unwrap();
                        let user_id = decoded_token.claims.get("sub").unwrap().as_str().unwrap();
                        return Ok(JwtMiddleware {
                            user_id: user_id.to_string(),
                        });
                    }
                    _ => unreachable!("this should be a RSA"),
                }
            } else {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "No matching JWK found for the given kid".to_string(),
                };
                return Err(ErrorUnauthorized(json_error));
            }
        })
    }
}
