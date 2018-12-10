#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_categories.graphql",
    response_derives = "Debug",
)]
pub struct GetCategoriesQuery;

pub use self::get_categories_query::*;
