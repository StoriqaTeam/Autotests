use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_user_delivery_address_full.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct CreateUserDeliveryAddressFullMutation;

pub use self::create_user_delivery_address_full_mutation::*;

pub type GraphqlRequestOutput = RustCreateUserDeliveryAddressFullCreateUserDeliveryAddressFull;

pub fn default_create_user_delivery_address_full_input() -> NewUserDeliveryAddressFullInput {
    NewUserDeliveryAddressFullInput {
        client_mutation_id: "".to_string(),
        user_id: 0,
        address_full: default_address_input(),
        is_priority: false,
    }
}

pub fn default_address_input() -> AddressInput {
    AddressInput {
        value: Some("Value".to_string()),
        country: Some("Country".to_string()),
        country_code: Some("Country Code".to_string()),
        administrative_area_level1: Some("Administrative Area Level 1".to_string()),
        administrative_area_level2: Some("Administrative Area Level 2".to_string()),
        locality: Some("Locality".to_string()),
        political: Some("Political".to_string()),
        postal_code: Some("Postal Code".to_string()),
        route: Some("Route".to_string()),
        street_number: Some("Street Number".to_string()),
        place_id: Some("Place ID".to_string()),
    }
}

impl GraphqlRequest for NewUserDeliveryAddressFullInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_user_delivery_address_full),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<NewUserDeliveryAddressFullInput> for serde_json::Value {
    fn from(val: NewUserDeliveryAddressFullInput) -> serde_json::Value {
        let request_body =
            CreateUserDeliveryAddressFullMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body)
            .expect("failed to serialize NewUserDeliveryAddressFullInput")
    }
}
