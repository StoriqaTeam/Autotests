use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_user_delivery_address_full.graphql"
)]
pub struct DeleteUserDeliveryAddressFullMutation;

pub use self::delete_user_delivery_address_full_mutation::*;

pub struct DeleteUserDeliveryAddressFullInput {
    pub id: i64,
}
pub type GraphqlRequestOutput = RustDeleteUserDeliveryAddressFullDeleteUserDeliveryAddressFull;

impl GraphqlRequest for DeleteUserDeliveryAddressFullInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_user_delivery_address_full),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteUserDeliveryAddressFullInput> for serde_json::Value {
    fn from(val: DeleteUserDeliveryAddressFullInput) -> serde_json::Value {
        let request_body =
            DeleteUserDeliveryAddressFullMutation::build_query(Variables { id: val.id });
        serde_json::to_value(request_body)
            .expect("failed to serialize DeleteUserDeliveryAddressFullInput")
    }
}
