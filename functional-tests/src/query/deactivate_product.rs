#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/deactivate_product.graphql"
)]
pub struct DeactivateProductMutation;

pub use self::deactivate_product_mutation::*;

pub fn default_update_base_product_input() -> DeactivateProductInput {
    DeactivateProductInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
    }
}
