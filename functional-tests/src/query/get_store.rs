#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_store.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct GetStoreQuery;

#[derive(Serialize, Debug, PartialEq)]
pub enum Visibility {
    Published,
    Active,
}

pub use self::get_store_query::*;
