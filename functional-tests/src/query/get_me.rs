use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_me.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct GetMeQuery;

pub use self::get_me_query::*;

pub struct GetMeInput;

impl GraphqlRequest for GetMeInput {
    type Output = Option<RustGetMeMe>;

    fn response(body: serde_json::Value) -> Result<Option<RustGetMeMe>, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.me),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetMeInput> for serde_json::Value {
    fn from(_val: GetMeInput) -> serde_json::Value {
        let request_body = GetMeQuery::build_query(Variables { });
        serde_json::to_value(request_body).expect("failed to serialize GetMeInput")
    }
}
