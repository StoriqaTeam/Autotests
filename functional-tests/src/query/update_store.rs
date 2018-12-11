#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_store.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct UpdateStoreMutation;

pub use self::update_store_mutation::*;

pub fn default_update_store_input() -> UpdateStoreInput {
    UpdateStoreInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
        name: None,
        short_description: None,
        long_description: None,
        slug: None,
        cover: None,
        logo: None,
        phone: None,
        email: None,
        slogan: None,
        instagram_url: None,
        twitter_url: None,
        facebook_url: None,
        default_language: None,
        rating: None,
        address_full: AddressInput {
            value: None,
            country: None,
            country_code: None,
            administrative_area_level1: None,
            administrative_area_level2: None,
            locality: None,
            political: None,
            postal_code: None,
            route: None,
            street_number: None,
            place_id: None,
        },
    }
}
