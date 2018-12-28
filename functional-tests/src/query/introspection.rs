use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/queries/introspection/introspection_schema.graphql",
    query_path = "graphql/queries/introspection/introspection_query.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct IntrospectionQuery;

pub use self::introspection_query::*;

pub struct IntrospectionInput;

pub const INTROSPECTION_RESPONSE: &'static str = include_str!("../../graphql/schema.json");

type GraphqlRequestOutput = ResponseData;

impl GraphqlRequest for IntrospectionInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<IntrospectionInput> for serde_json::Value {
    fn from(_val: IntrospectionInput) -> serde_json::Value {
        let request_body = IntrospectionQuery::build_query(Variables {});
        serde_json::to_value(request_body).expect("failed to serialize IntrospectionInput")
    }
}
