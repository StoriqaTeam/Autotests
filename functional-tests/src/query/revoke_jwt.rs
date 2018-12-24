use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/revoke_jwt.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct RevokeJwtMutation;

pub use self::revoke_jwt_mutation::*;

pub struct RevokeJwt;

impl GraphqlRequest for RevokeJwt {
    type Output = String;

    fn response(body: serde_json::Value) -> Result<Self::Output, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.revoke_jwt),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<RevokeJwt> for serde_json::Value {
    fn from(_val: RevokeJwt) -> serde_json::Value {
        let request_body = RevokeJwtMutation::build_query(Variables {});
        serde_json::to_value(request_body).expect("failed to serialize RevokeJwt")
    }
}
