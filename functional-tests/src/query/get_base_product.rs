#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_base_product.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct GetBaseProductQuery;

#[derive(Serialize, Debug, PartialEq)]
pub enum Visibility {
    Published,
    Active,
}

pub use self::get_base_product_query::*;
