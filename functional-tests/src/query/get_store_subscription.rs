use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_store_subscription.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct GetStoreSubscriptionQuery;

pub use self::get_store_subscription_query::*;

type GraphqlRequestOutput = Option<RustGetStoreSubscriptionStoreStoreSubscription>;

pub struct GetStoreSubscriptionInput {
    pub store_id: i64,
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone)]
pub enum Visibility {
    Published,
    Active,
}

impl GraphqlRequest for GetStoreSubscriptionInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.store.expect("Store not found").store_subscription),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetStoreSubscriptionInput> for serde_json::Value {
    fn from(val: GetStoreSubscriptionInput) -> serde_json::Value {
        let request_body = GetStoreSubscriptionQuery::build_query(Variables {
            id: val.store_id,
            visibility: Some(Visibility::Active),
        });
        serde_json::to_value(request_body).expect("failed to serialize GetStoreSubscriptionQuery")
    }
}
