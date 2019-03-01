use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/remove_delivery_method_from_cart.graphql"
)]
pub struct RemoveDeliveryMethodFromCartMutation;

pub use self::remove_delivery_method_from_cart_mutation::*;

pub fn default_set_delivery_method_in_cart_input() -> RemoveDeliveryMethodFromCartInput {
    RemoveDeliveryMethodFromCartInput {
        client_mutation_id: "".to_string(),
        product_id: 0,
    }
}

type GraphqlRequestOutput = RustRemoveDeliveryMethodFromCartRemoveDeliveryMethodFromCart;

impl GraphqlRequest for RemoveDeliveryMethodFromCartInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.remove_delivery_method_from_cart),
            (_, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<RemoveDeliveryMethodFromCartInput> for serde_json::Value {
    fn from(val: RemoveDeliveryMethodFromCartInput) -> serde_json::Value {
        let request_body =
            RemoveDeliveryMethodFromCartMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body)
            .expect("failed to serialize RemoveDeliveryMethodFromCartInput")
    }
}
