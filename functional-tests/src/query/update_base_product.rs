use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_base_product.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct UpdateBaseProductMutation;

pub use self::update_base_product_mutation::*;

pub fn default_update_base_product_input() -> UpdateBaseProductInput {
    UpdateBaseProductInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
        name: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "updated name".to_string(),
        }]),
        short_description: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "updated short_description".to_string(),
        }]),
        long_description: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "updated long_description".to_string(),
        }]),
        seo_title: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "updated seo_title".to_string(),
        }]),
        seo_description: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "updated seo_description".to_string(),
        }]),
        currency: Some(Currency::BTC),
        category_id: None,
        rating: Some(1.0),
        slug: Some("updated-slug".to_string()),
        length_cm: None,
        width_cm: None,
        height_cm: None,
        weight_g: None,
    }
}

type GraphqlRequestOutput = RustUpdateBaseProductUpdateBaseProduct;

impl GraphqlRequest for UpdateBaseProductInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_base_product),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdateBaseProductInput> for serde_json::Value {
    fn from(val: UpdateBaseProductInput) -> serde_json::Value {
        let request_body = UpdateBaseProductMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize UpdateBaseProductInput")
    }
}
