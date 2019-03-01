use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_user_delivery_addresses_full.graphql",
    response_derives = "Clone, Debug, PartialEq"
)]
pub struct GetUserDeliveryAddressesFullQuery;

pub use self::get_user_delivery_addresses_full_query::*;

pub struct GetUserDeliveryAddressesInput;
pub type GraphqlRequestOutput = Option<RustGetUserDeliveryAddressesFullMe>;

impl GraphqlRequest for GetUserDeliveryAddressesInput {
    type Output = Option<RustGetUserDeliveryAddressesFullMe>;

    fn response(
        body: serde_json::Value,
    ) -> Result<Option<RustGetUserDeliveryAddressesFullMe>, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.me),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetUserDeliveryAddressesInput> for serde_json::Value {
    fn from(_val: GetUserDeliveryAddressesInput) -> serde_json::Value {
        let request_body = GetUserDeliveryAddressesFullQuery::build_query(Variables {});
        serde_json::to_value(request_body)
            .expect("failed to serialize GetUserDeliveryAddressesInput")
    }
}
