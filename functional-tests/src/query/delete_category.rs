use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_category.graphql"
)]
pub struct DeleteCategoryMutation;

pub use self::delete_category_mutation::*;

pub fn default_delete_category_input() -> DeleteCategoryInput {
    DeleteCategoryInput {
        cat_id: 0,
        client_mutation_id: "".to_string(),
    }
}

type GraphqlRequestOutput = RustDeleteCategoryDeleteCategory;

impl GraphqlRequest for DeleteCategoryInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_category),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteCategoryInput> for serde_json::Value {
    fn from(val: DeleteCategoryInput) -> serde_json::Value {
        let request_body = DeleteCategoryMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize DeleteCategoryInput")
    }
}
