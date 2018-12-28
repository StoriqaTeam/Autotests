extern crate failure;
extern crate functional_tests;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate graphql_client;

mod basic_tests;
mod cart;
mod check_schema;
mod healthcheck;
