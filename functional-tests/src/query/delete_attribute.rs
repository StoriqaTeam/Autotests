#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_attribute.graphql"
)]
pub struct DeleteAttributeMutation;

pub use self::delete_attribute_mutation::*;

pub fn default_delete_attribute_input() -> DeleteAttributeInput {
    DeleteAttributeInput {
        id: "".to_string(),
        client_mutation_id: "".to_string(),
    }
}
