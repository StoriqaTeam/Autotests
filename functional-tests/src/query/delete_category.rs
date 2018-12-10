#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_category.graphql"
)]
pub struct DeleteCategoryMutation;

pub use self::delete_category_mutation::*;

pub fn default_delete_category_input() -> DeleteCategoryInput {
    DeleteCategoryInput {
        cat_id: 0,
        client_mutation_id: "".to_string(),
    }
}
