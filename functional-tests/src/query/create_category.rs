use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_category.graphql",
    response_derives = "Debug"
)]
pub struct CreateCategoryMutation;

pub use self::create_category_mutation::*;

pub fn default_create_category_input() -> CreateCategoryInput {
    let slug: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
    let slug = slug.to_ascii_lowercase();

    CreateCategoryInput {
        client_mutation_id: "".to_string(),
        name: vec![TranslationInput {
            lang: Language::EN,
            text: "Category".to_string(),
        }],
        meta_field: Some(r#"{"ui_element": "Combobox"}"#.to_string()),
        parent_id: 0,
        slug: Some(slug),
    }
}

type GraphqlRequestOutput = RustCreateCategoryCreateCategory;

impl GraphqlRequest for CreateCategoryInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_category),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<CreateCategoryInput> for serde_json::Value {
    fn from(val: CreateCategoryInput) -> serde_json::Value {
        let request_body = CreateCategoryMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize CreateCategoryMutation")
    }
}
