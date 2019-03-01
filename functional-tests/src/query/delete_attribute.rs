use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_attribute.graphql",
    response_derives = "Debug"
)]
pub struct DeleteAttributeMutation;

pub use self::delete_attribute_mutation::*;

pub fn default_delete_attribute_input() -> DeleteAttributeInput {
    DeleteAttributeInput {
        id: "".to_string(),
        client_mutation_id: "".to_string(),
    }
}

type GraphqlRequestOutput = RustDeleteAttributeDeleteAttribute;

impl GraphqlRequest for DeleteAttributeInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_attribute),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteAttributeInput> for serde_json::Value {
    fn from(val: DeleteAttributeInput) -> serde_json::Value {
        let request_body = DeleteAttributeMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize DeleteAttributeInput")
    }
}
