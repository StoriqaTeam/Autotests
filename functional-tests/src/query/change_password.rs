use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/change_password.graphql"
)]
pub struct ChangePasswordMutation;

pub use self::change_password_mutation::*;

pub fn default_change_password_input() -> ChangePasswordInput {
    ChangePasswordInput {
        client_mutation_id: "".to_string(),
        old_password: "Qwerty123".to_string(),
        new_password: "321ytrewQ".to_string(),
    }
}

type GraphqlRequestOutput = RustChangePasswordChangePassword;

impl GraphqlRequest for ChangePasswordInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.change_password),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<ChangePasswordInput> for serde_json::Value {
    fn from(val: ChangePasswordInput) -> serde_json::Value {
        let request_body = ChangePasswordMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize ChangePasswordInput")
    }
}
