use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/apply_password_reset.graphql"
)]
pub struct ApplyPasswordResetMutation;

pub use self::apply_password_reset_mutation::*;

pub fn default_apply_password_reset_input() -> ResetApply {
    ResetApply {
        client_mutation_id: "".to_string(),
        token: "".to_string(),
        password: "321ytrewQ".to_string(),
        project: None,
    }
}

type GraphqlRequestOutput = RustApplyPasswordResetApplyPasswordReset;

impl GraphqlRequest for ResetApply {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.apply_password_reset),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<ResetApply> for serde_json::Value {
    fn from(val: ResetApply) -> serde_json::Value {
        let request_body = ApplyPasswordResetMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize ResetApply")
    }
}
