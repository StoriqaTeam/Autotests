#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_attribute.graphql",
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
