#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_product.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct UpdateProductMutation;

pub use self::update_product_mutation::*;

pub fn default_update_product_with_attributes_input() -> UpdateProductWithAttributesInput {
    UpdateProductWithAttributesInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
        product: Some(default_update_product_input()),
        attributes: None,
    }
}

pub fn default_update_product_input() -> UpdateProduct {
    UpdateProduct {
        discount: None,
        photo_main: None,
        additional_photos: None,
        vendor_code: None,
        cashback: None,
        price: None,
        pre_order: None,
        pre_order_days: None,
    }
}
