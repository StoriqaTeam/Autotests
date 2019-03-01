use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_attribute.graphql"
)]
pub struct CreateAttributeMutation;

pub use self::create_attribute_mutation::*;

pub fn default_create_attribute_input() -> CreateAttributeInput {
    CreateAttributeInput {
        client_mutation_id: "".to_string(),
        name: vec![TranslationInput {
            lang: Language::EN,
            text: "Category".to_string(),
        }],
        value_type: AttributeType::STR,
        meta_field: None,
        values: None,
    }
}

pub fn create_attribute_value(
    code: impl Into<String>,
    text_en: impl Into<String>,
    text_ru: impl Into<String>,
) -> CreateAttributeValueWithAttributeInput {
    CreateAttributeValueWithAttributeInput {
        code: code.into(),
        translations: Some(vec![
            TranslationInput {
                lang: Language::EN,
                text: text_en.into(),
            },
            TranslationInput {
                lang: Language::RU,
                text: text_ru.into(),
            },
        ]),
    }
}

impl GraphqlRequest for CreateAttributeInput {
    type Output = RustCreateAttributeCreateAttribute;

    fn response(
        body: serde_json::Value,
    ) -> Result<RustCreateAttributeCreateAttribute, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_attribute),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<CreateAttributeInput> for serde_json::Value {
    fn from(val: CreateAttributeInput) -> serde_json::Value {
        let request_body = CreateAttributeMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize CreateAttributeInput")
    }
}
