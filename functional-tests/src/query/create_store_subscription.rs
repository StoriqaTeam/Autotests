use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_store_subscription.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct CreateStoreSubscriptionMutation;

pub use self::create_store_subscription_mutation::*;

type GraphqlRequestOutput = RustCreateStoreSubscriptionCreateStoreSubscription;

pub fn default_create_store_subscription_input() -> CreateStoreSubscriptionInput {
    CreateStoreSubscriptionInput {
        client_mutation_id: "".to_string(),
        store_id: 0,
        currency: Currency::EUR,
    }
}

impl GraphqlRequest for CreateStoreSubscriptionInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_store_subscription),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<CreateStoreSubscriptionInput> for serde_json::Value {
    fn from(val: CreateStoreSubscriptionInput) -> serde_json::Value {
        let request_body = CreateStoreSubscriptionMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body)
            .expect("failed to serialize CreateStoreSubscriptionMutation")
    }
}
