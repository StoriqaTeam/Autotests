use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_categories_with_products.graphql",
    response_derives = "Debug"
)]
pub struct GetCategoriesWithProductsQuery;

pub use self::get_categories_with_products_query::*;

pub struct GetCategoriesWithProductsInput;

pub type GraphqlRequestOutput = Option<RustGetCategoriesWithProductsCategories>;

impl GraphqlRequest for GetCategoriesWithProductsInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.categories),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetCategoriesWithProductsInput> for serde_json::Value {
    fn from(_val: GetCategoriesWithProductsInput) -> serde_json::Value {
        let request_body = GetCategoriesWithProductsQuery::build_query(Variables {});
        serde_json::to_value(request_body)
            .expect("failed to serialize GetCategoriesWithProductsInput")
    }
}
