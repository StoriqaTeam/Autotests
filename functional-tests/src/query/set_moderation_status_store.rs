use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/set_moderation_status_store.graphql"
)]
pub struct SetModerationStatusStoreMutation;

pub use self::set_moderation_status_store_mutation::*;

pub fn default_set_moderation_status_store_input() -> StoreModerateInput {
    StoreModerateInput {
        id: "".to_string(),
        status: Status::PUBLISHED,
    }
}

type GraphqlRequestOutput = RustSetModerationStatusStoreSetModerationStatusStore;

impl GraphqlRequest for StoreModerateInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.set_moderation_status_store),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<StoreModerateInput> for serde_json::Value {
    fn from(val: StoreModerateInput) -> serde_json::Value {
        let request_body = SetModerationStatusStoreMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize StoreModerateInput")
    }
}
