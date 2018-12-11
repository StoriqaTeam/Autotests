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
        name: None,
        short_description: None,
        long_description: None,
        seo_title: None,
        seo_description: None,
        currency: None,
        category_id: None,
        rating: None,
        slug: None,
        length_cm: None,
        width_cm: None,
        height_cm: None,
        weight_g: None,
    }
}
