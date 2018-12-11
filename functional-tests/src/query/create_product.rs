#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_product.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct CreateProductMutation;

pub use self::create_product_mutation::*;

pub fn default_create_product_input() -> CreateProductWithAttributesInput {
    CreateProductWithAttributesInput {
        client_mutation_id: "".to_string(),
        product: default_new_product(),
        attributes: vec![],
    }
}

pub fn default_new_product() -> NewProduct {
    NewProduct {
        client_mutation_id: None,
        base_product_id: None,
        discount: None,
        photo_main: None,
        additional_photos: None,
        vendor_code: "1".to_string(),
        cashback: None,
        price: 1f64, // TODO: validate > 0.0 need test
        pre_order: None,
        pre_order_days: None,
    }
}
