#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_attributes.graphql"
)]
pub struct GetAttributesQuery;

pub use self::get_attributes_query::*;
