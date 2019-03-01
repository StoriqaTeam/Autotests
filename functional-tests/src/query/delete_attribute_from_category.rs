use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_attribute_from_category.graphql"
)]
pub struct DeleteAttributeFromCategoryMutation;

pub use self::delete_attribute_from_category_mutation::*;

pub fn default_delete_attribute_from_category_input() -> DeleteAttributeFromCategory {
    DeleteAttributeFromCategory {
        client_mutation_id: "".to_string(),
        cat_id: 0,
        attr_id: 0,
    }
}

type GraphqlRequestOutput = RustDeleteAttributeFromCategoryDeleteAttributeFromCategory;

impl GraphqlRequest for DeleteAttributeFromCategory {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_attribute_from_category),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteAttributeFromCategory> for serde_json::Value {
    fn from(val: DeleteAttributeFromCategory) -> serde_json::Value {
        let request_body =
            DeleteAttributeFromCategoryMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize DeleteAttributeFromCategory")
    }
}
