use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_jwt_by_email.graphql"
)]
pub struct GetJwtByEmailMutation;

pub use self::get_jwt_by_email_mutation::*;

pub fn default_create_jwt_email_input() -> CreateJWTEmailInput {
    CreateJWTEmailInput {
        client_mutation_id: "".to_string(),
        email: "user@mail.com".to_string(),
        password: "Qwerty123".to_string(),
    }
}

type GraphqlRequestOutput = RustGetJwtByEmailGetJwtByEmail;

impl GraphqlRequest for CreateJWTEmailInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.get_jwt_by_email),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<CreateJWTEmailInput> for serde_json::Value {
    fn from(val: CreateJWTEmailInput) -> serde_json::Value {
        let request_body = GetJwtByEmailMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize CreateJWTEmailInput")
    }
}
