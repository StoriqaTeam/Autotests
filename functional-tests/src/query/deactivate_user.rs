use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/deactivate_user.graphql"
)]
pub struct DeactivateUserMutation;

pub use self::deactivate_user_mutation::*;

pub fn default_deactivate_user_input() -> DeactivateUserInput {
    DeactivateUserInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
    }
}

type GraphqlRequestOutput = RustDeactivateUserDeactivateUser;

impl GraphqlRequest for DeactivateUserInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.deactivate_user),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeactivateUserInput> for serde_json::Value {
    fn from(val: DeactivateUserInput) -> serde_json::Value {
        let request_body = DeactivateUserMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize DeactivateUserInput")
    }
}
