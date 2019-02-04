extern crate actix;
extern crate actix_web;
extern crate functional_tests;
extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use actix_web::http::Method;
use actix_web::{middleware, pred, server, App, HttpResponse};

use functional_tests::config::Config;

mod routes {
    use actix_web::http::StatusCode;
    use actix_web::{
        AsyncResponder, Error as ActixError, HttpMessage, HttpRequest, HttpResponse, Result,
    };
    use futures::Future;

    use functional_tests::config::Config;
    use functional_tests::context::{
        DataContext, HeathCheckMicroservice, MicroserviceDataContextImpl, VerifyUserEmail,
    };

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
        pub config: Config,
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

    pub fn clear_all_data(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
        let config = req.state().config.clone();
        let context = MicroserviceDataContextImpl::new(config, reqwest::Client::new());

        match context.clear_all_data() {
            Ok(_) => Response::ok(),
            Err(_) => Response::could_not_clear_data(),
        }
        .to_http_response()
    }

    pub fn verify_user_email(
        req: &HttpRequest<AppState>,
    ) -> Box<Future<Item = HttpResponse, Error = ActixError>> {
        let config = req.state().config.clone();
        let context = MicroserviceDataContextImpl::new(config, reqwest::Client::new());

        req.json()
            .from_err()
            .and_then(move |data: VerifyUserEmail| {
                match context.verify_user_email(&data.email) {
                    Ok(_) => Response::ok(),
                    Err(_) => Response::new(StatusCode::INTERNAL_SERVER_ERROR),
                }
                .to_http_response()
            })
            .responder()
    }

    pub fn microservice_healthcheck(
        req: &HttpRequest<AppState>,
    ) -> Box<Future<Item = HttpResponse, Error = ActixError>> {
        let config = req.state().config.clone();
        let context = MicroserviceDataContextImpl::new(config, reqwest::Client::new());

        req.json()
            .from_err()
            .and_then(move |data: HeathCheckMicroservice| {
                match context.microservice_healthcheck(data.microservice) {
                    Ok(_) => Response::ok(),
                    Err(_) => Response::new(StatusCode::INTERNAL_SERVER_ERROR),
                }
                .to_http_response()
            })
            .responder()
    }

    pub fn healthcheck(_req: &HttpRequest<AppState>) -> Result<HttpResponse> {
        Response::ok().to_http_response()
    }

    pub fn not_found(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
        Response::not_found(req.path().to_string()).to_http_response()
    }
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = actix::System::new("main");
    let config = Config::new().expect("Cannot read config.");
    let _ = server::new(move || {
        App::with_state(routes::AppState {
            config: config.clone(),
        })
        // enable logger
        .middleware(middleware::Logger::default())
        .resource("/testtools/clear_all_data", |r| {
            r.method(Method::POST).f(routes::clear_all_data)
        })
        .resource("/testtools/verify_user_email", |r| {
            r.method(Method::POST).f(routes::verify_user_email)
        })
        .resource("/testtools/microservice_healthcheck", |r| {
            r.method(Method::POST).f(routes::microservice_healthcheck)
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
