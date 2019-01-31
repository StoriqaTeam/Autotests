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

use functional_tests::config::Config;
use functional_tests::context::TestContext;

mod routes {
    use actix_web::http::StatusCode;
    use actix_web::{HttpRequest, HttpResponse, Result};

    use functional_tests::context::TestContext;

    #[derive(Serialize)]
    #[serde(tag = "type", rename_all = "camelCase")]
    pub enum Error {
        NotFound { url: String },
        CouldNotClearData {},
    }

    #[derive(Serialize)]
    pub struct Response {
        pub status: StatusCodeWrapper,
        pub errors: Vec<Error>,
    }

    pub struct AppState {
        pub context: TestContext,
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

        pub fn could_not_clear_data() -> Response {
            Response::with_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::CouldNotClearData {},
            )
        }
    }

    pub fn clear(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
        let ref context = req.state().context;

        match context.clear_all_data() {
            Ok(_) => Response::ok(),
            Err(_) => Response::could_not_clear_data(),
        }
        .to_http_response()
    }

    pub fn healthcheck(_req: &HttpRequest<AppState>) -> Result<HttpResponse> {
        Response::ok().to_http_response()
    }

    pub fn not_found(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
        Response::not_found(req.path().to_string()).to_http_response()
    }
}

fn main() {
    let sys = actix::System::new("main");
    let config = Config::with_env("base").expect("Cannot read config 'base'");
    let context = TestContext::with_config(config);
    let _ = server::new(move || {
        App::with_state(routes::AppState {
            context: context.clone(),
        })
        .resource("/testtools/clear", |r| {
            r.method(Method::POST).f(routes::clear)
        })
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
