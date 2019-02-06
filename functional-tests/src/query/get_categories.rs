use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_categories.graphql",
    response_derives = "Debug"
)]
pub struct GetCategoriesQuery;

pub use self::get_categories_query::*;

pub struct GetCategoriesInput;

pub type GraphqlRequestOutput = Option<RustGetCategoriesAllCategories>;

impl GraphqlRequest for GetCategoriesInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.all_categories),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetCategoriesInput> for serde_json::Value {
    fn from(_val: GetCategoriesInput) -> serde_json::Value {
        let request_body = GetCategoriesQuery::build_query(Variables {});
        serde_json::to_value(request_body).expect("failed to serialize GetCategoriesInput")
    }
}
