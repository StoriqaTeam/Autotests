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
        variants: vec![
            CreateProductWithAttributesInput {
                client_mutation_id: "".to_string(),
                product: NewProduct {
                    client_mutation_id: None,
                    base_product_id: None,
                    discount: Some(30.0),
                    photo_main: Some("photo".to_string()),
                    additional_photos: Some(vec!["additional_photo_1".to_string(), "additional_photo_2".to_string()]),
                    vendor_code: "vendor_code".to_string(),
                    cashback: Some(10.0),
                    price: 100.0,
                    pre_order: Some(false),
                    pre_order_days: Some(100),
                },
                attributes: vec![],
            }
        ],
        length_cm: Some(10),
        width_cm: Some(10),
        height_cm: Some(10),
        weight_g: Some(1000),
    }
}
