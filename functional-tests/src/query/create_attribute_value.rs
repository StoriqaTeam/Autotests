use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_attribute_value.graphql"
)]
pub struct CreateAttributeValueMutation;

pub use self::create_attribute_value_mutation::*;

pub fn default_create_attribute_value_input() -> CreateAttributeValueInput {
    CreateAttributeValueInput {
        client_mutation_id: "".to_string(),
        raw_attribute_id: 0,
        code: "attribute_value_code".to_string(),
        translations: Some(vec![
            TranslationInput {
                lang: Language::EN,
                text: "attribute value".to_string(),
            },
            TranslationInput {
                lang: Language::CH,
                text: "attribute value china".to_string(),
            },
        ]),
    }
}

impl GraphqlRequest for CreateAttributeValueInput {
    type Output = RustCreateAttributeValueCreateAttributeValue;

    fn response(
        body: serde_json::Value,
    ) -> Result<RustCreateAttributeValueCreateAttributeValue, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_attribute_value),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<CreateAttributeValueInput> for serde_json::Value {
    fn from(val: CreateAttributeValueInput) -> serde_json::Value {
        let request_body = CreateAttributeValueMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize CreateAttributeValueInput")
    }
}
