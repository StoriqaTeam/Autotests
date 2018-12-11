#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_attribute_from_category.graphql"
)]
pub struct DeleteAttributeFromCategoryMutation;

pub use self::delete_attribute_from_category_mutation::*;

pub fn default_delete_attribute_from_category_input() -> DeleteAttributeFromCategory {
    DeleteAttributeFromCategory {
        client_mutation_id: "".to_string(),
        cat_id: 0,
        attr_id: 0,
    }
}
