use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/clear_cart.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct ClearCartQuery;

pub use self::clear_cart_query::*;

pub struct ClearCartInput;
pub type GraphqlRequestOutput = RustClearCartClearCart;

impl GraphqlRequest for ClearCartInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.clear_cart),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<ClearCartInput> for serde_json::Value {
    fn from(_val: ClearCartInput) -> serde_json::Value {
        let request_body = ClearCartQuery::build_query(Variables {});
        serde_json::to_value(request_body).expect("failed to serialize ClearCartInput")
    }
}
