use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_store_subscription.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct UpdateStoreSubscriptionMutation;

pub use self::update_store_subscription_mutation::*;

type GraphqlRequestOutput = RustUpdateStoreSubscriptionUpdateStoreSubscription;

pub fn default_update_store_subscription_input() -> UpdateStoreSubscriptionInput {
    UpdateStoreSubscriptionInput {
        client_mutation_id: "".to_string(),
        store_id: 0,
        currency: None,
        status: None,
    }
}

impl GraphqlRequest for UpdateStoreSubscriptionInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_store_subscription),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdateStoreSubscriptionInput> for serde_json::Value {
    fn from(val: UpdateStoreSubscriptionInput) -> serde_json::Value {
        let request_body = UpdateStoreSubscriptionMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body)
            .expect("failed to serialize UpdateStoreSubscriptionMutation")
    }
}
