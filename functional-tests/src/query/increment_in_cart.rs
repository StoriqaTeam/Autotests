use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/increment_in_cart.graphql"
)]
pub struct IncrementInCartMutation;

pub use self::increment_in_cart_mutation::*;

pub fn default_increment_in_cart_input() -> IncrementInCartInput {
    IncrementInCartInput {
        client_mutation_id: "".to_string(),
        product_id: 1,
        value: None,
    }
}

type GraphqlRequestOutput = Option<RustIncrementInCartIncrementInCart>;

impl GraphqlRequest for IncrementInCartInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.increment_in_cart),
            (_, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<IncrementInCartInput> for serde_json::Value {
    fn from(val: IncrementInCartInput) -> serde_json::Value {
        let request_body = IncrementInCartMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize IncrementInCartInput")
    }
}
