#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_attribute_value.graphql"
)]
pub struct UpdateAttributeValueMutation;

pub use self::update_attribute_value_mutation::*;

pub fn default_create_attribute_value_input() -> UpdateAttributeValueInput {
    UpdateAttributeValueInput {
        raw_id: 0,
        client_mutation_id: "".to_string(),
        raw_attribute_id: 0,
        code: Some("update_attribute_value_code".to_string()),
        translations: Some(vec![
            TranslationInput {
                lang: Language::EN,
                text: "update attribute value".to_string(),
            },
            TranslationInput {
                lang: Language::CH,
                text: "update attribute value china".to_string(),
            },
        ]),
    }
}
