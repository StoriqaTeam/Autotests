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
