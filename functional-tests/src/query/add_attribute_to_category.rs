use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/add_attribute_to_category.graphql"
)]
pub struct AddAttributeToCategoryMutation;

pub use self::add_attribute_to_category_mutation::*;

pub fn default_add_attribute_to_category_input() -> AddAttributeToCategoryInput {
    AddAttributeToCategoryInput {
        client_mutation_id: "".to_string(),
        cat_id: 0,
        attr_id: 0,
    }
}

type GraphqlRequestOutput = RustAddAttributeToCategoryAddAttributeToCategory;

impl GraphqlRequest for AddAttributeToCategoryInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.add_attribute_to_category),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<AddAttributeToCategoryInput> for serde_json::Value {
    fn from(val: AddAttributeToCategoryInput) -> serde_json::Value {
        let request_body = AddAttributeToCategoryMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize AddAttributeToCategoryInput")
    }
}
