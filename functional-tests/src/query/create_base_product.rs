use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_base_product.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct CreateBaseProductMutation;

pub use self::create_base_product_mutation::*;

pub fn default_create_base_product_input() -> CreateBaseProductInput {
    CreateBaseProductInput {
        client_mutation_id: "".to_string(),
        name: vec![TranslationInput {
            lang: Language::EN,
            text: "Base product".to_string(),
        }],
        store_id: 0,
        short_description: vec![TranslationInput {
            lang: Language::EN,
            text: "Base product short description".to_string(),
        }],
        long_description: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "Base product long description".to_string(),
        }]),
        seo_title: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "Base product seo title".to_string(),
        }]),
        seo_description: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "Base product seo description".to_string(),
        }]),
        currency: Currency::STQ,
        category_id: 0,
        slug: Some("base-product-slug".to_string()),
        length_cm: Some(10),
        width_cm: Some(10),
        height_cm: Some(10),
        weight_g: Some(1000),
    }
}

type GraphqlRequestOutput = RustCreateBaseProductCreateBaseProduct;

impl GraphqlRequest for CreateBaseProductInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_base_product),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<CreateBaseProductInput> for serde_json::Value {
    fn from(val: CreateBaseProductInput) -> serde_json::Value {
        let request_body = CreateBaseProductMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize CreateBaseProductMutation")
    }
}
