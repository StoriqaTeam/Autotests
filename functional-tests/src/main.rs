extern crate actix;
extern crate actix_web;
extern crate functional_tests;
extern crate hyper;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use actix_web::http::Method;
use actix_web::{pred, server, App, HttpResponse};

mod routes {
    use actix_web::http::StatusCode;
    use actix_web::{HttpRequest, HttpResponse, Result};

    use functional_tests::config::Config;
    use functional_tests::context::TestContext;

    #[derive(Serialize)]
    #[serde(tag = "type", rename_all = "camelCase")]
    pub enum Error {
        NotFound { url: String },
        CouldNotReadConfig {},
        CouldNotClearData {},
    }

    #[derive(Serialize)]
    pub struct Response {
        pub status: StatusCodeWrapper,
        pub errors: Vec<Error>,
    }

    pub struct StatusCodeWrapper(StatusCode);

    impl serde::ser::Serialize for StatusCodeWrapper {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_u16(self.0.as_u16())
        }
    }

    impl Response {
        pub fn to_http_response(&self) -> Result<HttpResponse> {
            Ok(HttpResponse::build(self.status.0)
                .content_type("application/json; charset=utf-8")
                .json(self))
        }

        pub fn new(status: StatusCode) -> Response {
            Response::with_errors(status, vec![])
        }

        pub fn with_error(status: StatusCode, error: Error) -> Response {
            Response::with_errors(status, vec![error])
        }

        pub fn with_errors(status: StatusCode, errors: Vec<Error>) -> Response {
            Response {
                status: StatusCodeWrapper(status),
                errors,
            }
        }

        pub fn ok() -> Response {
            Response::new(StatusCode::OK)
        }

        pub fn not_found(url: String) -> Response {
            Response::with_error(StatusCode::NOT_FOUND, Error::NotFound { url })
        }

        pub fn could_not_read_config() -> Response {
            Response::with_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::CouldNotReadConfig {},
            )
        }

        pub fn could_not_clear_data() -> Response {
            Response::with_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::CouldNotClearData {},
            )
        }
    }

    pub fn clear(_req: &HttpRequest) -> Result<HttpResponse> {
        let config = match Config::with_env("base") {
            Ok(config) => config,
            Err(_) => return Response::could_not_read_config().to_http_response(),
        };
        let context = TestContext::with_config(config);

        match context.clear_all_data() {
            Ok(_) => Response::ok(),
            Err(_) => Response::could_not_clear_data(),
        }
        .to_http_response()
    }

    pub fn healthcheck(_req: &HttpRequest) -> Result<HttpResponse> {
        Response::ok().to_http_response()
    }

    pub fn not_found(req: &HttpRequest) -> Result<HttpResponse> {
        Response::not_found(req.path().to_string()).to_http_response()
    }
}

fn main() {
    let sys = actix::System::new("main");
    let _ = server::new(|| {
        App::new()
            .resource("/clear", |r| r.method(Method::POST).f(routes::clear))
            .resource("/healthcheck", |r| {
                r.method(Method::GET).f(routes::healthcheck)
            })
            .default_resource(|r| {
                // 404 for GET request
                r.method(Method::GET).f(routes::not_found);

                // all requests that are not `GET`
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|_| HttpResponse::MethodNotAllowed());
            })
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to 0.0.0.0:8000")
    .shutdown_timeout(0)
    .start();

    println!("Listening on 0.0.0.0:8000...");
    let _ = sys.run();
}
