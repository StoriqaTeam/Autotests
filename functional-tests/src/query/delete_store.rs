#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_store.graphql"
)]
pub struct DeleteStoreMutation;

pub use self::delete_store_mutation::*;
