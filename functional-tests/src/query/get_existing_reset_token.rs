use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_existing_reset_token.graphql"
)]
pub struct GetExistingResetTokenQuery;

pub use self::get_existing_reset_token_query::*;

pub fn default_get_existing_reset_token_query_input() -> ExistingResetTokenInput {
    ExistingResetTokenInput {
        user_id: 1,
        token_type: TokenTypeInput::EMAIL_VERIFY,
    }
}

type GraphqlRequestOutput = RustGetExistingResetTokenExistingResetToken;

impl GraphqlRequest for ExistingResetTokenInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.existing_reset_token),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<ExistingResetTokenInput> for serde_json::Value {
    fn from(val: ExistingResetTokenInput) -> serde_json::Value {
        let request_body = GetExistingResetTokenQuery::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize ExistingResetTokenInput")
    }
}
