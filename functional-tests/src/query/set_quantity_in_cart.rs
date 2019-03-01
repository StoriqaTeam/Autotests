use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/set_quantity_in_cart.graphql"
)]
pub struct SetQuantityInCartMutation;

pub use self::set_quantity_in_cart_mutation::*;

pub fn default_set_quantity_in_cart_input() -> SetQuantityInCartInput {
    SetQuantityInCartInput {
        client_mutation_id: "".to_string(),
        product_id: 0,
        value: 0,
    }
}

type GraphqlRequestOutput = RustSetQuantityInCartSetQuantityInCart;

impl GraphqlRequest for SetQuantityInCartInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data
                .set_quantity_in_cart
                .expect("SetQuantityInCartInput failed to execute query: response is None")),
            (_, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<SetQuantityInCartInput> for serde_json::Value {
    fn from(val: SetQuantityInCartInput) -> serde_json::Value {
        let request_body = SetQuantityInCartMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize SetQuantityInCartInput")
    }
}
