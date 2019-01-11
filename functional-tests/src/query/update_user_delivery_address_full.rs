use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_user_delivery_address_full.graphql"
)]
pub struct UpdateUserDeliveryAddressFullMutation;

pub use self::update_user_delivery_address_full_mutation::*;

pub type GraphqlRequestOutput = RustUpdateUserDeliveryAddressFullUpdateUserDeliveryAddressFull;

pub fn default_update_user_delivery_address_full_input() -> UpdateUserDeliveryAddressFullInput {
    UpdateUserDeliveryAddressFullInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
        address_full: default_address_input(),
        is_priority: None,
    }
}

pub fn default_address_input() -> AddressInput {
    AddressInput {
        value: None,
        country: None,
        country_code: None,
        administrative_area_level1: None,
        administrative_area_level2: None,
        locality: None,
        political: None,
        postal_code: None,
        route: None,
        street_number: None,
        place_id: None,
    }
}

impl GraphqlRequest for UpdateUserDeliveryAddressFullInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_user_delivery_address_full),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdateUserDeliveryAddressFullInput> for serde_json::Value {
    fn from(val: UpdateUserDeliveryAddressFullInput) -> serde_json::Value {
        let request_body =
            UpdateUserDeliveryAddressFullMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body)
            .expect("failed to serialize UpdateUserDeliveryAddressFullInput")
    }
}
