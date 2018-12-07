#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_attribute_value.graphql",
)]
pub struct DeleteAttributeValueMutation;

pub use self::delete_attribute_value_mutation::*;

pub fn default_delete_attribute_value_input() -> DeleteAttributeValueInput {
    DeleteAttributeValueInput {
        raw_id: 0,
        client_mutation_id: "".to_string(),
    }
}
