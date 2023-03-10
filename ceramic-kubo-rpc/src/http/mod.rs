//! Provides an http implementation of the Kubo RPC methods.
use std::net;

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    web, App, HttpResponse, HttpServer,
};
use serde::Serialize;
use tracing_actix_web::TracingLogger;

use crate::{error::Error, IpfsDep};

mod dag;
mod swarm;

#[derive(Clone)]
struct AppState<T>
where
    T: IpfsDep,
{
    api: T,
}

/// Start the Kubo RPC mimic server.
///
/// Block until shutdown.
/// Automatically registers shutdown listeners for interrupt and kill signals.
/// See https://actix.rs/docs/server/#graceful-shutdown
pub async fn serve<T, A>(api: T, addrs: A) -> std::io::Result<()>
where
    T: IpfsDep + Send + Clone + 'static,
    A: net::ToSocketAddrs,
{
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(AppState { api: api.clone() }))
            .service(
                web::scope("/api/v0")
                    .service(dag::scope::<T>())
                    .service(swarm::scope::<T>()),
            )
    })
    .bind(addrs)?
    .run()
    .await
}

#[derive(Serialize)]
struct ErrorJson<'a> {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Code")]
    pub code: i32,
    #[serde(rename = "Type")]
    pub typ: &'a str,
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let err = ErrorJson {
            message: self.to_string(),
            code: 0,
            typ: "error",
        };
        let data = serde_json::to_string(&err).unwrap();
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(data)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Invalid(_) => StatusCode::BAD_REQUEST,
            Error::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::{
        body::{self, MessageBody},
        dev::ServiceResponse,
        test, web, App,
    };
    use expect_test::Expect;
    use unimock::Unimock;

    /// Test helper function to build a application server
    pub async fn build_server(
        mock: impl IpfsDep + 'static,
    ) -> impl actix_web::dev::Service<
        actix_http::Request,
        Response = ServiceResponse,
        Error = actix_web::Error,
    > {
        test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { api: mock }))
                .service(super::dag::scope::<Unimock>())
                .service(super::swarm::scope::<Unimock>()),
        )
        .await
    }

    /// Test helper function to assert a JSON reponse body
    pub async fn assert_body_json<B>(body: B, expect: Expect)
    where
        B: MessageBody,
        <B as MessageBody>::Error: std::fmt::Debug,
    {
        let body_json: serde_json::Value =
            serde_json::from_slice(body::to_bytes(body).await.unwrap().as_ref())
                .expect("response body should be valid json");
        let pretty_json = serde_json::to_string_pretty(&body_json).unwrap();
        expect.assert_eq(&pretty_json);
    }

    /// Test helper function to assert a binary reponse body
    pub async fn assert_body_binary<B>(body: B, expect: Expect)
    where
        B: MessageBody,
        <B as MessageBody>::Error: std::fmt::Debug,
    {
        let bytes = hex::encode(&body::to_bytes(body).await.unwrap());
        expect.assert_eq(&bytes);
    }
}
