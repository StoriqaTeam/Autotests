#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/deactivate_user.graphql"
)]
pub struct DeactivateUserMutation;

pub use self::deactivate_user_mutation::*;

pub fn default_deactivate_user_input() -> DeactivateUserInput {
    DeactivateUserInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
    }
}
