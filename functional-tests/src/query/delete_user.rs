#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_user.graphql"
)]
pub struct DeleteUserMutation;

pub use self::delete_user_mutation::*;
