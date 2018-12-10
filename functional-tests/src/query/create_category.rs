#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_category.graphql"
)]
pub struct CreateCategoryMutation;

pub use self::create_category_mutation::*;

pub fn default_create_category_input() -> CreateCategoryInput {
    CreateCategoryInput {
        client_mutation_id: "".to_string(),
        name: vec![TranslationInput {
            lang: Language::EN,
            text: "Category".to_string(),
        }],
        meta_field: Some(r#"{"ui_element": "Combobox"}"#.to_string()),
        parent_id: 0,
        slug: Some("category-slug".to_string()),
    }
}
