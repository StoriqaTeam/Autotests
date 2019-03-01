use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/verify_email.graphql"
)]
pub struct VerifyEmailMutation;

pub use self::verify_email_mutation::*;

pub fn default_verify_email_input() -> VerifyEmailApply {
    VerifyEmailApply {
        client_mutation_id: "".to_string(),
        token: "".to_string(),
        project: Some(Project::MARKET_PLACE),
    }
}

type GraphqlRequestOutput = RustVerifyEmailVerifyEmail;

impl GraphqlRequest for VerifyEmailApply {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.verify_email),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<VerifyEmailApply> for serde_json::Value {
    fn from(val: VerifyEmailApply) -> serde_json::Value {
        let request_body = VerifyEmailMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize VerifyEmailApply")
    }
}
