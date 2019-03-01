use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/add_in_cart.graphql"
)]
pub struct AddInCartMutation;

pub use self::add_in_cart_mutation::*;

pub fn default_add_in_cart_input() -> AddInCartInput {
    AddInCartInput {
        client_mutation_id: "".to_string(),
        product_id: 1,
        value: None,
        shipping_id: None,
    }
}

type GraphqlRequestOutput = Option<RustAddInCartAddInCart>;

impl GraphqlRequest for AddInCartInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.add_in_cart),
            (_, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<AddInCartInput> for serde_json::Value {
    fn from(val: AddInCartInput) -> serde_json::Value {
        let request_body = AddInCartMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize AddInCartInput")
    }
}
