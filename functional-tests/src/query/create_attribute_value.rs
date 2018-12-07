#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_attribute_value.graphql",
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
