use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/request_password_reset.graphql"
)]
pub struct RequestPasswordResetMutation;

pub use self::request_password_reset_mutation::*;

pub fn default_change_password_input() -> ResetRequest {
    ResetRequest {
        client_mutation_id: "".to_string(),
        email: "".to_string(),
        device: None,
        project: None,
    }
}

type GraphqlRequestOutput = RustRequestPasswordResetRequestPasswordReset;

impl GraphqlRequest for ResetRequest {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.request_password_reset),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<ResetRequest> for serde_json::Value {
    fn from(val: ResetRequest) -> serde_json::Value {
        let request_body = RequestPasswordResetMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize ResetRequest")
    }
}
