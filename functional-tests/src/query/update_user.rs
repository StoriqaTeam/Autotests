use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_user.graphql"
)]
pub struct UpdateUserMutation;

pub use self::update_user_mutation::*;

pub fn default_update_user_input() -> UpdateUserInput {
    UpdateUserInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
        is_active: None,
        phone: Some("9876543".to_string()),
        first_name: Some("updated user".to_string()),
        last_name: Some("updated userovsky".to_string()),
        middle_name: Some("updated middleName".to_string()),
        gender: Some(Gender::MALE),
        birthdate: Some("1990-01-01".to_string()),
        avatar: Some("updated avatar".to_string()),
    }
}

type GraphqlRequestOutput = RustUpdateUserUpdateUser;

impl GraphqlRequest for UpdateUserInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_user),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdateUserInput> for serde_json::Value {
    fn from(val: UpdateUserInput) -> serde_json::Value {
        let request_body = UpdateUserMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize UpdateUserInput")
    }
}
