#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_category.graphql",
)]
pub struct UpdateCategoryMutation;

pub use self::update_category_mutation::*;

pub fn default_update_category_input() -> UpdateCategoryInput {
    UpdateCategoryInput {
        id: "".to_string(),
        client_mutation_id: "".to_string(),
        name: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "Update category".to_string(),
        }]),
        meta_field: Some(r#"{"ui_element": "ColorPicker"}"#.to_string()),
        parent_id: Some(1),
        slug: Some("update-category-slug".to_string()),
        level: Some(2),
    }
}
