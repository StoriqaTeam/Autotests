use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_custom_attribute.graphql"
)]
pub struct CreateCustomAttributeMutation;

pub use self::create_custom_attribute_mutation::*;

pub type GraphqlRequestOutput = RustCreateCustomAttributeCreateCustomAttribute;

pub fn default_create_custom_attribute_input() -> NewCustomAttributeInput {
    NewCustomAttributeInput {
        client_mutation_id: "".to_string(),
        attribute_id: 0,
        base_product_id: 0,
    }
}

impl GraphqlRequest for NewCustomAttributeInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_custom_attribute),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<NewCustomAttributeInput> for serde_json::Value {
    fn from(val: NewCustomAttributeInput) -> serde_json::Value {
        let request_body = CreateCustomAttributeMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize NewCustomAttributeInput")
    }
}
