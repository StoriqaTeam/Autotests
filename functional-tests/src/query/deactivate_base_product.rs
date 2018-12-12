#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/deactivate_base_product.graphql"
)]
pub struct DeactivateBaseProductMutation;

pub use self::deactivate_base_product_mutation::*;

pub fn default_deactivate_base_product_input() -> DeactivateBaseProductInput {
    DeactivateBaseProductInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
    }
}
