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
