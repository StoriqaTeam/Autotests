use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/set_delivery_method_in_cart.graphql"
)]
pub struct SetDeliveryMethodInCartMutation;

pub use self::set_delivery_method_in_cart_mutation::*;

pub fn default_set_delivery_method_in_cart_input() -> SetDeliveryMethodInCartInput {
    SetDeliveryMethodInCartInput {
        client_mutation_id: "".to_string(),
        product_id: 0,
        company_package_id: None,
        shipping_id: 0,
    }
}

type GraphqlRequestOutput = RustSetDeliveryMethodInCartSetDeliveryMethodInCart;

impl GraphqlRequest for SetDeliveryMethodInCartInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.set_delivery_method_in_cart),
            (_, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<SetDeliveryMethodInCartInput> for serde_json::Value {
    fn from(val: SetDeliveryMethodInCartInput) -> serde_json::Value {
        let request_body = SetDeliveryMethodInCartMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body)
            .expect("failed to serialize SetDeliveryMethodInCartInput")
    }
}
