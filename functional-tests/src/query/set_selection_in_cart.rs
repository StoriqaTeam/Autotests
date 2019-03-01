use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/set_selection_in_cart.graphql"
)]
pub struct SetSelectionInCartMutation;

pub use self::set_selection_in_cart_mutation::*;

pub fn default_set_selection_in_cart_input() -> SetSelectionInCartInput {
    SetSelectionInCartInput {
        client_mutation_id: "".to_string(),
        product_id: 0,
        value: false,
    }
}

type GraphqlRequestOutput = RustSetSelectionInCartSetSelectionInCart;

impl GraphqlRequest for SetSelectionInCartInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data
                .set_selection_in_cart
                .expect("SetSelectionInCartInput failed to execute query: response is None")),
            (_, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<SetSelectionInCartInput> for serde_json::Value {
    fn from(val: SetSelectionInCartInput) -> serde_json::Value {
        let request_body = SetSelectionInCartMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize SetSelectionInCartInput")
    }
}
