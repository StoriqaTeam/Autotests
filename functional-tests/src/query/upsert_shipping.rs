use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "graphql/schema.json",
query_path = "graphql/queries/upsert_shipping.graphql"
)]
pub struct UpsertShippingMutation;

pub use self::upsert_shipping_mutation::*;

pub type GraphqlRequestInput = NewShippingInput;
pub type GraphqlRequestOutput = RustUpsertShippingUpsertShipping;

pub fn default_graphql_request_input() -> GraphqlRequestInput {
    GraphqlRequestInput {
        client_mutation_id: "".to_string(),
        base_product_id: 0,
        store_id: 0,
        pickup: Some(default_new_pickups_input()),
        local: vec![],
        international: vec![]
    }
}

pub fn default_new_pickups_input() -> NewPickupsInput {
    NewPickupsInput { pickup: false, price: None }
}

impl GraphqlRequest for GraphqlRequestInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.upsert_shipping),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<NewShippingInput> for serde_json::Value {
    fn from(val: NewShippingInput) -> serde_json::Value {
        let request_body = UpsertShippingMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize UpsertShippingInput")
    }
}
