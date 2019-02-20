use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_base_product_with_variants.graphql"
)]
pub struct CreateBaseProductWithVariantsMutation;

pub use self::create_base_product_with_variants_mutation::*;

pub fn default_create_base_product_with_variants_input() -> NewBaseProductWithVariantsInput {
    NewBaseProductWithVariantsInput {
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
        selected_attributes: vec![],
        variants: vec![CreateProductWithAttributesInput {
            client_mutation_id: "".to_string(),
            product: NewProduct {
                client_mutation_id: None,
                base_product_id: None,
                discount: Some(0.3),
                photo_main: Some("photo".to_string()),
                additional_photos: Some(vec![
                    "additional_photo_1".to_string(),
                    "additional_photo_2".to_string(),
                ]),
                vendor_code: "vendor_code".to_string(),
                cashback: Some(0.1),
                price: 100.0,
                pre_order: Some(false),
                pre_order_days: Some(100),
            },
            attributes: vec![],
            quantity: Some(999),
        }],
        length_cm: Some(10),
        width_cm: Some(10),
        height_cm: Some(10),
        weight_g: Some(1000),
    }
}

type GraphqlRequestOutput = RustCreateBaseProductWithVariantsCreateBaseProductWithVariants;

impl GraphqlRequest for NewBaseProductWithVariantsInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_base_product_with_variants),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<NewBaseProductWithVariantsInput> for serde_json::Value {
    fn from(val: NewBaseProductWithVariantsInput) -> serde_json::Value {
        let request_body =
            CreateBaseProductWithVariantsMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body)
            .expect("failed to serialize CreateBaseProductWithVariantsMutation")
    }
}
