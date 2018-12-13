use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_category.graphql"
)]
pub struct UpdateCategoryMutation;

pub use self::update_category_mutation::*;

pub fn default_update_category_input() -> UpdateCategoryInput {
    UpdateCategoryInput {
        id: "".to_string(),
        client_mutation_id: "".to_string(),
        name: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "Update category".to_string(),
        }]),
        meta_field: Some(r#"{"ui_element": "ColorPicker"}"#.to_string()),
        parent_id: Some(1),
        slug: Some("update-category-slug".to_string()),
        level: Some(2),
    }
}

type GraphqlRequestOutput = RustUpdateCategoryUpdateCategory;

impl GraphqlRequest for UpdateCategoryInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_category),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdateCategoryInput> for serde_json::Value {
    fn from(val: UpdateCategoryInput) -> serde_json::Value {
        let request_body = UpdateCategoryMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize UpdateCategoryInput")
    }
}
