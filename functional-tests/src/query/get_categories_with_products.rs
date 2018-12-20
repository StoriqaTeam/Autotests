#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_categories_with_products.graphql",
    response_derives = "Debug"
)]
pub struct GetCategoriesWithProductsQuery;

pub use self::get_categories_with_products_query::*;
