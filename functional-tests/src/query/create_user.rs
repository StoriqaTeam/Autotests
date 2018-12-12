use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_user.graphql"
)]
pub struct CreateUserMutation;

pub use self::create_user_mutation::*;

pub fn default_create_user_input() -> CreateUserInput {
    CreateUserInput {
        additional_data: None,
        client_mutation_id: "".to_string(),
        device: None,
        email: "user@mail.com".to_string(),
        first_name: "User".to_string(),
        last_name: "Userovsky".to_string(),
        password: "Qwerty123".to_string(),
        project: None,
    }
}

impl GraphqlRequest for CreateUserInput {
    type Output = RustCreateUserCreateUser;

    fn response(body: serde_json::Value) -> Result<RustCreateUserCreateUser, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_user),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<CreateUserInput> for serde_json::Value {
    fn from(val: CreateUserInput) -> serde_json::Value {
        let request_body = CreateUserMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize CreateUserInput")
    }
}
