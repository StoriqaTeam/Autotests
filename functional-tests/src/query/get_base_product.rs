#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_base_product.graphql"
)]
pub struct GetBaseProductQuery;

#[derive(Serialize)]
pub enum Visibility {
    Published,
    Active,
}

pub use self::get_base_product_query::*;
