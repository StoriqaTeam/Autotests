use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "graphql/schema.json",
query_path = "graphql/queries/delete_custom_attribute.graphql"
)]
pub struct DeleteCustomAttributeMutation;

pub use self::delete_custom_attribute_mutation::*;

pub type GraphqlRequestOutput = RustDeleteCustomAttributeDeleteCustomAttribute;

pub fn default_delete_custom_attribute_input() -> DeleteCustomAttributeInput {
    DeleteCustomAttributeInput {
        client_mutation_id: "".to_string(),
        custom_attribute_id: 0
    }
}

impl GraphqlRequest for DeleteCustomAttributeInput {
    type Output = GraphqlRequestOutput;

    fn response(
        body: serde_json::Value,
    ) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_custom_attribute),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteCustomAttributeInput> for serde_json::Value {
    fn from(val: DeleteCustomAttributeInput) -> serde_json::Value {
        let request_body = DeleteCustomAttributeMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize DeleteCustomAttributeInput")
    }
}
