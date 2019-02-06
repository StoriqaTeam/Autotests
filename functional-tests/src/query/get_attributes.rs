use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_attributes.graphql"
)]
pub struct GetAttributesQuery;

pub use self::get_attributes_query::*;

pub struct GetAttributesInput;

pub type GraphqlRequestOutput = Option<Vec<RustGetAttributesAttributes>>;

impl GraphqlRequest for GetAttributesInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.attributes),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetAttributesInput> for serde_json::Value {
    fn from(_val: GetAttributesInput) -> serde_json::Value {
        let request_body = GetAttributesQuery::build_query(Variables {});
        serde_json::to_value(request_body).expect("failed to serialize GetAttributesInput")
    }
}
