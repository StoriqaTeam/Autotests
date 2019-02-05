use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_user.graphql"
)]
pub struct DeleteUserMutation;

pub use self::delete_user_mutation::*;

pub struct DeleteUserInput {
    pub user_id: i64,
}

pub type GraphqlRequestOutput = RustDeleteUserDeleteUser;

impl GraphqlRequest for DeleteUserInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_user),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteUserInput> for serde_json::Value {
    fn from(val: DeleteUserInput) -> serde_json::Value {
        let request_body = DeleteUserMutation::build_query(Variables {
            user_id: val.user_id,
        });
        serde_json::to_value(request_body).expect("failed to serialize DeleteUserInput")
    }
}
