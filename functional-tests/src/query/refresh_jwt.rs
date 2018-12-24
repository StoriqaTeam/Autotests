use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/refresh_jwt.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct RefreshJwtMutation;

pub use self::refresh_jwt_mutation::*;

pub struct RefreshJwt;

impl GraphqlRequest for RefreshJwt {
    type Output = String;

    fn response(body: serde_json::Value) -> Result<Self::Output, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.refresh_jwt),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<RefreshJwt> for serde_json::Value {
    fn from(_val: RefreshJwt) -> serde_json::Value {
        let request_body = RefreshJwtMutation::build_query(Variables {});
        serde_json::to_value(request_body).expect("failed to serialize RefreshJwt")
    }
}
