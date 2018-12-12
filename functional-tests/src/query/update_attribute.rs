use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_attribute.graphql"
)]
pub struct UpdateAttributeMutation;

pub use self::update_attribute_mutation::*;

pub fn default_update_attribute_input() -> UpdateAttributeInput {
    UpdateAttributeInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
        name: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "Update category".to_string(),
        }]),
        meta_field: None,
    }
}

type GraphqlRequestOutput = RustUpdateAttributeUpdateAttribute;

impl GraphqlRequest for UpdateAttributeInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_attribute),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdateAttributeInput> for serde_json::Value {
    fn from(val: UpdateAttributeInput) -> serde_json::Value {
        let request_body = UpdateAttributeMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize UpdateAttributeInput")
    }
}
