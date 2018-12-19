use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/send_base_product_to_moderation.graphql"
)]
pub struct SendBaseProductToModerationMutation;

pub use self::send_base_product_to_moderation_mutation::*;

pub struct SendBaseProductToModerationInput {
    pub raw_id: i64,
}

pub fn default_send_base_product_to_moderation_input() -> SendBaseProductToModerationInput {
    SendBaseProductToModerationInput { raw_id: 0 }
}

type GraphqlRequestOutput = RustSendBaseProductToModerationSendBaseProductToModeration;

impl GraphqlRequest for SendBaseProductToModerationInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.send_base_product_to_moderation),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<SendBaseProductToModerationInput> for serde_json::Value {
    fn from(val: SendBaseProductToModerationInput) -> serde_json::Value {
        let request_body =
            SendBaseProductToModerationMutation::build_query(Variables { id: val.raw_id });
        serde_json::to_value(request_body)
            .expect("failed to serialize SendBaseProductToModerationInput")
    }
}
